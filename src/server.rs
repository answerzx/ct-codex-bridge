use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

const SESSION_COOKIE: &str = "ct_codex_bridge";
const CODEX_ICON_SVG: &str = include_str!("../assets/codex-color.svg");
const APPLE_TOUCH_ICON_PNG: &[u8] = include_bytes!("../assets/apple-touch-icon.png");

#[derive(Clone)]
struct AppState {
    switch_lock: Arc<Mutex<()>>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionStatus {
    authenticated: bool,
    password_configured: bool,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

pub async fn serve(port: u16) -> Result<(), String> {
    let state = AppState {
        switch_lock: Arc::new(Mutex::new(())),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/favicon.svg", get(favicon_svg))
        .route("/icon.svg", get(favicon_svg))
        .route("/apple-touch-icon.png", get(apple_touch_icon))
        .route("/api/session/status", get(session_status))
        .route("/api/session/login", post(login))
        .route("/api/session/logout", post(logout))
        .route("/api/codex/accounts", get(accounts))
        .route("/api/codex/status", get(codex_status))
        .route("/api/codex/switch", post(switch_account))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|error| format!("bind {addr}: {error}"))?;
    println!("Codex Account Bridge listening on http://{addr}");
    axum::serve(listener, app)
        .await
        .map_err(|error| format!("server error: {error}"))
}

async fn index() -> Html<&'static str> {
    Html(crate::ui::INDEX_HTML)
}

async fn favicon_svg() -> Response {
    (
        [
            (header::CONTENT_TYPE, "image/svg+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        CODEX_ICON_SVG,
    )
        .into_response()
}

async fn apple_touch_icon() -> Response {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        APPLE_TOUCH_ICON_PNG.to_vec(),
    )
        .into_response()
}

async fn session_status(headers: HeaderMap) -> Json<SessionStatus> {
    Json(SessionStatus {
        authenticated: is_authorized(&headers),
        password_configured: crate::config::BridgeConfig::load_or_default()
            .ok()
            .and_then(|cfg| cfg.password_hash)
            .is_some(),
    })
}

async fn login(Json(payload): Json<LoginRequest>) -> Response {
    let mut cfg = match crate::config::BridgeConfig::load_or_default() {
        Ok(cfg) => cfg,
        Err(message) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, message),
    };
    let Some(hash) = cfg.password_hash.as_deref() else {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "password is not configured; run `ct-codex-bridge setup-password` first".to_string(),
        );
    };

    if !crate::auth::verify_password(&hash, &payload.password) {
        return error_response(StatusCode::UNAUTHORIZED, "invalid password".to_string());
    }

    let secret = match cfg
        .session_secret
        .clone()
        .filter(|value| !value.trim().is_empty())
    {
        Some(secret) => secret,
        None => {
            let secret = crate::auth::generate_session_secret();
            cfg.session_secret = Some(secret.clone());
            if let Err(message) = cfg.save() {
                return error_response(StatusCode::INTERNAL_SERVER_ERROR, message);
            }
            secret
        }
    };

    let token = crate::auth::create_signed_session(&secret, now_unix());
    let cookie = format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        crate::auth::SESSION_TTL_SECONDS
    );
    let mut response = Json(serde_json::json!({ "ok": true })).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).expect("valid session cookie"),
    );
    response
}

async fn logout() -> Response {
    let mut response = Json(serde_json::json!({ "ok": true })).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static("ct_codex_bridge=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"),
    );
    response
}

async fn accounts(headers: HeaderMap) -> Response {
    if !is_authorized(&headers) {
        return error_response(StatusCode::UNAUTHORIZED, "unauthorized".to_string());
    }
    let paths = match crate::codex::ResolvedPaths::new() {
        Ok(paths) => paths,
        Err(message) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, message),
    };
    match crate::codex::list_accounts(&paths) {
        Ok(accounts) => Json(accounts).into_response(),
        Err(message) => error_response(StatusCode::INTERNAL_SERVER_ERROR, message),
    }
}

async fn codex_status(headers: HeaderMap) -> Response {
    if !is_authorized(&headers) {
        return error_response(StatusCode::UNAUTHORIZED, "unauthorized".to_string());
    }
    let paths = match crate::codex::ResolvedPaths::new() {
        Ok(paths) => paths,
        Err(message) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, message),
    };
    Json(crate::codex::status(&paths)).into_response()
}

async fn switch_account(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<crate::codex::SwitchRequest>,
) -> Response {
    if !is_authorized(&headers) {
        return error_response(StatusCode::UNAUTHORIZED, "unauthorized".to_string());
    }

    let _guard = state.switch_lock.lock().await;
    let paths = match crate::codex::ResolvedPaths::new() {
        Ok(paths) => paths,
        Err(message) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, message),
    };
    match crate::codex::switch_account(&paths, payload) {
        Ok(result) => Json(result).into_response(),
        Err(message) => error_response(StatusCode::BAD_REQUEST, message),
    }
}

fn is_authorized(headers: &HeaderMap) -> bool {
    let Some(token) = session_token(headers) else {
        return false;
    };
    let Some(secret) = crate::config::BridgeConfig::load_or_default()
        .ok()
        .and_then(|cfg| cfg.session_secret)
        .filter(|value| !value.trim().is_empty())
    else {
        return false;
    };
    crate::auth::validate_signed_session(&secret, &token, now_unix())
}

fn session_token(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
    crate::auth::parse_cookie(Some(cookie), SESSION_COOKIE)
}

fn error_response(status: StatusCode, message: String) -> Response {
    (status, Json(ErrorBody { error: message })).into_response()
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
