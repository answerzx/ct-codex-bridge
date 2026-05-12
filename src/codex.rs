use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const CT_DATA_DIR: &str = ".antigravity_cockpit";
const CODEX_AUTH_PROJECTION_FILE: &str = ".cockpit_codex_auth.json";
const CODEX_KEYCHAIN_SERVICE: &str = "Codex Auth";

#[derive(Debug, Clone)]
pub struct ResolvedPaths {
    pub home: PathBuf,
    pub ct_data_dir: PathBuf,
    pub account_index_path: PathBuf,
    pub accounts_dir: PathBuf,
    pub codex_home: PathBuf,
    pub codex_app_path: PathBuf,
}

impl ResolvedPaths {
    pub fn new() -> Result<Self, String> {
        let home =
            dirs::home_dir().ok_or_else(|| "unable to resolve home directory".to_string())?;
        let codex_home = resolve_codex_home(&home);
        Ok(Self::from_home_and_codex_home(home, codex_home))
    }

    pub fn from_home_and_codex_home(home: PathBuf, codex_home: PathBuf) -> Self {
        let ct_data_dir = home.join(CT_DATA_DIR);
        let account_index_path = ct_data_dir.join("codex_accounts.json");
        let accounts_dir = ct_data_dir.join("codex_accounts");
        let codex_app_path = resolve_codex_app_path(&home)
            .unwrap_or_else(|| PathBuf::from("/Applications/Codex.app"));
        Self {
            home,
            ct_data_dir,
            account_index_path,
            accounts_dir,
            codex_home,
            codex_app_path,
        }
    }
}

fn resolve_codex_home(home: &Path) -> PathBuf {
    if let Ok(raw) = std::env::var("CODEX_HOME") {
        let trimmed = raw.trim().trim_matches('"').trim_matches('\'').trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    home.join(".codex")
}

fn resolve_codex_app_path(home: &Path) -> Option<PathBuf> {
    let config_path = home.join(CT_DATA_DIR).join("config.json");
    let content = fs::read_to_string(config_path).ok()?;
    let value: Value = serde_json::from_str(&content).ok()?;
    value
        .get("codex_app_path")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexAccountIndex {
    pub version: Option<String>,
    #[serde(default)]
    pub accounts: Vec<CodexAccountSummary>,
    pub current_account_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexAccountSummary {
    pub id: String,
    pub email: String,
    pub plan_type: Option<String>,
    #[serde(default)]
    pub subscription_active_until: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub last_used: i64,
}

#[derive(Debug, Clone)]
pub struct CodexAccountDetail {
    pub raw: Value,
    pub id: String,
    pub email: String,
    pub auth_mode: String,
    pub openai_api_key: Option<String>,
    pub api_base_url: Option<String>,
    pub api_provider_mode: Option<String>,
    pub api_provider_id: Option<String>,
    pub api_provider_name: Option<String>,
    pub plan_type: Option<String>,
    pub subscription_active_until: Option<String>,
    pub account_id: Option<String>,
    pub account_name: Option<String>,
    pub account_structure: Option<String>,
    pub account_note: Option<String>,
    pub tokens: Option<CodexTokens>,
    pub token_generation: u64,
    pub requires_reauth: bool,
    pub reauth_reason: Option<String>,
    pub quota: Option<CodexQuota>,
    pub quota_error: Option<CodexQuotaErrorInfo>,
    pub usage_updated_at: Option<i64>,
    pub created_at: i64,
    pub last_used: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexTokens {
    pub id_token: String,
    pub access_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexQuota {
    pub hourly_percentage: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_reset_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_window_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_window_present: Option<bool>,
    pub weekly_percentage: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_reset_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_window_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_window_present: Option<bool>,
    #[serde(default, skip_serializing)]
    pub raw_data: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexQuotaErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountView {
    pub id: String,
    pub email: String,
    pub plan_type: Option<String>,
    pub subscription_active_until: Option<String>,
    pub account_name: Option<String>,
    pub account_structure: Option<String>,
    pub account_note: Option<String>,
    pub auth_mode: String,
    pub is_current: bool,
    pub requires_reauth: bool,
    pub reauth_reason: Option<String>,
    #[serde(rename = "hasOAuthSnapshot")]
    pub has_oauth_snapshot: bool,
    pub has_api_key: bool,
    pub can_switch: bool,
    pub quota: Option<CodexQuota>,
    pub quota_error: Option<CodexQuotaErrorInfo>,
    pub usage_updated_at: Option<i64>,
    pub created_at: i64,
    pub last_used: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsResponse {
    pub current_account_id: Option<String>,
    pub accounts: Vec<AccountView>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub codex_home: String,
    pub ct_data_dir: String,
    pub codex_app_path: String,
    pub codex_app_exists: bool,
    pub codex_running: bool,
    pub current_projection: Option<ManagedProjection>,
    pub keychain_account: String,
    pub keychain_entry_present: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ManagedProjection {
    pub version: u32,
    pub writer: String,
    pub account_id: String,
    pub email: String,
    pub token_generation: u64,
    pub written_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchResponse {
    pub account: AccountView,
    pub restarted: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchRequest {
    pub account_id: String,
    #[serde(default = "default_restart")]
    pub restart: bool,
}

fn default_restart() -> bool {
    true
}

impl CodexAccountDetail {
    pub fn from_value(
        value: Value,
        fallback: Option<&CodexAccountSummary>,
    ) -> Result<Self, String> {
        let id = read_string(&value, &["id"])
            .or_else(|| fallback.map(|item| item.id.clone()))
            .ok_or_else(|| "account detail is missing id".to_string())?;
        let email = read_string(&value, &["email"])
            .or_else(|| fallback.map(|item| item.email.clone()))
            .ok_or_else(|| format!("account {id} is missing email"))?;
        let auth_mode = read_string(&value, &["auth_mode", "authMode"]).unwrap_or_else(|| {
            if extract_openai_api_key(&value).is_some() {
                "apikey".to_string()
            } else {
                "oauth".to_string()
            }
        });
        let tokens = value
            .get("tokens")
            .and_then(|tokens| serde_json::from_value::<CodexTokens>(tokens.clone()).ok());

        Ok(Self {
            id,
            email,
            auth_mode,
            openai_api_key: extract_openai_api_key(&value),
            api_base_url: read_string(&value, &["api_base_url", "apiBaseUrl"]),
            api_provider_mode: read_string(&value, &["api_provider_mode", "apiProviderMode"]),
            api_provider_id: read_string(&value, &["api_provider_id", "apiProviderId"]),
            api_provider_name: read_string(&value, &["api_provider_name", "apiProviderName"]),
            plan_type: read_string(&value, &["plan_type", "planType"])
                .or_else(|| fallback.and_then(|item| item.plan_type.clone())),
            subscription_active_until: read_string(
                &value,
                &["subscription_active_until", "subscriptionActiveUntil"],
            )
            .or_else(|| fallback.and_then(|item| item.subscription_active_until.clone())),
            account_id: read_string(&value, &["account_id", "accountId"]),
            account_name: read_string(&value, &["account_name", "accountName"]),
            account_structure: read_string(&value, &["account_structure", "accountStructure"]),
            account_note: read_string(&value, &["account_note", "accountNote"]),
            tokens,
            token_generation: value
                .get("token_generation")
                .or_else(|| value.get("tokenGeneration"))
                .and_then(Value::as_u64)
                .unwrap_or(0),
            requires_reauth: value
                .get("requires_reauth")
                .or_else(|| value.get("requiresReauth"))
                .and_then(Value::as_bool)
                .unwrap_or(false),
            reauth_reason: read_string(&value, &["reauth_reason", "reauthReason"]),
            quota: value
                .get("quota")
                .and_then(|quota| serde_json::from_value::<CodexQuota>(quota.clone()).ok()),
            quota_error: value
                .get("quota_error")
                .or_else(|| value.get("quotaError"))
                .and_then(|quota_error| {
                    serde_json::from_value::<CodexQuotaErrorInfo>(quota_error.clone()).ok()
                }),
            usage_updated_at: value
                .get("usage_updated_at")
                .or_else(|| value.get("usageUpdatedAt"))
                .and_then(Value::as_i64),
            created_at: value
                .get("created_at")
                .or_else(|| value.get("createdAt"))
                .and_then(Value::as_i64)
                .or_else(|| fallback.map(|item| item.created_at))
                .unwrap_or(0),
            last_used: value
                .get("last_used")
                .or_else(|| value.get("lastUsed"))
                .and_then(Value::as_i64)
                .or_else(|| fallback.map(|item| item.last_used))
                .unwrap_or(0),
            raw: value,
        })
    }

    fn is_api_key_auth(&self) -> bool {
        self.auth_mode.trim().eq_ignore_ascii_case("apikey")
            || (self.tokens.is_none() && self.openai_api_key.is_some())
    }

    fn has_switch_material(&self) -> bool {
        if self.is_api_key_auth() {
            return self
                .openai_api_key
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty());
        }
        self.tokens.as_ref().is_some_and(|tokens| {
            !tokens.id_token.trim().is_empty() && !tokens.access_token.trim().is_empty()
        })
    }

    fn to_view(&self, current_account_id: Option<&str>) -> AccountView {
        AccountView {
            id: self.id.clone(),
            email: self.email.clone(),
            plan_type: self.plan_type.clone(),
            subscription_active_until: self.subscription_active_until.clone(),
            account_name: self.account_name.clone(),
            account_structure: self.account_structure.clone(),
            account_note: self.account_note.clone(),
            auth_mode: if self.is_api_key_auth() {
                "apikey".to_string()
            } else {
                "oauth".to_string()
            },
            is_current: current_account_id == Some(self.id.as_str()),
            requires_reauth: self.requires_reauth,
            reauth_reason: self.reauth_reason.clone(),
            has_oauth_snapshot: self.tokens.is_some(),
            has_api_key: self.openai_api_key.is_some(),
            can_switch: self.has_switch_material(),
            quota: self.quota.clone(),
            quota_error: self.quota_error.clone(),
            usage_updated_at: self.usage_updated_at,
            created_at: self.created_at,
            last_used: self.last_used,
        }
    }
}

pub fn list_accounts(paths: &ResolvedPaths) -> Result<AccountsResponse, String> {
    let index = load_index(paths)?;
    let mut accounts = Vec::new();
    for summary in &index.accounts {
        match load_account_detail(paths, &summary.id, Some(summary)) {
            Ok(detail) => accounts.push(detail.to_view(index.current_account_id.as_deref())),
            Err(error) => {
                eprintln!("skip unreadable account {}: {error}", summary.id);
            }
        }
    }
    accounts.sort_by(|left, right| right.last_used.cmp(&left.last_used));
    Ok(AccountsResponse {
        current_account_id: index.current_account_id,
        accounts,
    })
}

pub fn status(paths: &ResolvedPaths) -> StatusResponse {
    let projection = read_projection(paths).ok().flatten();
    let keychain_account = codex_keychain_account(&paths.codex_home);
    StatusResponse {
        codex_home: paths.codex_home.to_string_lossy().to_string(),
        ct_data_dir: paths.ct_data_dir.to_string_lossy().to_string(),
        codex_app_path: paths.codex_app_path.to_string_lossy().to_string(),
        codex_app_exists: normalize_macos_app_root(&paths.codex_app_path)
            .map(|path| path.exists())
            .unwrap_or_else(|| paths.codex_app_path.exists()),
        codex_running: crate::process::codex_pids()
            .map(|pids| !pids.is_empty())
            .unwrap_or(false),
        current_projection: projection,
        keychain_entry_present: keychain_entry_present(&keychain_account),
        keychain_account,
    }
}

pub fn switch_account(
    paths: &ResolvedPaths,
    request: SwitchRequest,
) -> Result<SwitchResponse, String> {
    let account_id = request.account_id.trim();
    if account_id.is_empty() {
        return Err("accountId is required".to_string());
    }

    let mut index = load_index(paths)?;
    let summary = index
        .accounts
        .iter()
        .find(|item| item.id == account_id)
        .cloned();
    if summary.is_none() {
        return Err(format!("account does not exist in CT index: {account_id}"));
    }

    let mut detail = load_account_detail(paths, account_id, summary.as_ref())?;
    if !detail.has_switch_material() {
        return Err(format!(
            "account {} does not contain enough saved Codex credential material",
            detail.email
        ));
    }

    let rollback = RollbackSnapshot::capture(paths, &detail.id)?;
    backup_before_switch(paths)?;

    let result = (|| {
        write_account_bundle_to_codex_home(paths, &detail)?;

        index.current_account_id = Some(detail.id.clone());
        save_index(paths, &index)?;

        let now = now_timestamp();
        detail.last_used = now;
        if let Some(object) = detail.raw.as_object_mut() {
            object.insert("last_used".to_string(), json!(now));
        }
        save_account_detail(paths, &detail)?;

        let restarted = if request.restart {
            crate::process::restart_codex(&paths.codex_app_path)?
        } else {
            false
        };

        Ok(SwitchResponse {
            account: detail.to_view(Some(detail.id.as_str())),
            restarted,
        })
    })();

    if result.is_err() {
        let _ = rollback.restore();
    }

    result
}

pub fn load_index(paths: &ResolvedPaths) -> Result<CodexAccountIndex, String> {
    let content = fs::read_to_string(&paths.account_index_path)
        .map_err(|error| format!("read {}: {error}", paths.account_index_path.display()))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("parse {}: {error}", paths.account_index_path.display()))
}

fn save_index(paths: &ResolvedPaths, index: &CodexAccountIndex) -> Result<(), String> {
    let content = serde_json::to_string_pretty(index)
        .map_err(|error| format!("serialize account index: {error}"))?;
    write_string_atomic(&paths.account_index_path, &content)
}

fn load_account_detail(
    paths: &ResolvedPaths,
    account_id: &str,
    fallback: Option<&CodexAccountSummary>,
) -> Result<CodexAccountDetail, String> {
    let path = paths.accounts_dir.join(format!("{account_id}.json"));
    let content =
        fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&content)
        .map_err(|error| format!("parse {}: {error}", path.display()))?;
    CodexAccountDetail::from_value(value, fallback)
}

fn save_account_detail(paths: &ResolvedPaths, detail: &CodexAccountDetail) -> Result<(), String> {
    let path = paths.accounts_dir.join(format!("{}.json", detail.id));
    let content = serde_json::to_string_pretty(&detail.raw)
        .map_err(|error| format!("serialize account detail: {error}"))?;
    write_string_atomic(&path, &content)
}

fn write_account_bundle_to_codex_home(
    paths: &ResolvedPaths,
    account: &CodexAccountDetail,
) -> Result<(), String> {
    fs::create_dir_all(&paths.codex_home)
        .map_err(|error| format!("create {}: {error}", paths.codex_home.display()))?;
    let auth = build_auth_file_value(account)?;
    let auth_content = serde_json::to_string_pretty(&auth)
        .map_err(|error| format!("serialize auth.json: {error}"))?;
    write_string_atomic(&paths.codex_home.join("auth.json"), &auth_content)?;

    if !account.is_api_key_auth() {
        write_codex_keychain(&paths.codex_home, &auth)?;
    }

    write_managed_projection(paths, account)?;
    write_api_provider_to_config_toml(&paths.codex_home, account)?;
    Ok(())
}

fn build_auth_file_value(account: &CodexAccountDetail) -> Result<Value, String> {
    if account.is_api_key_auth() {
        let api_key = account
            .openai_api_key
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "API key account is missing OPENAI_API_KEY".to_string())?;
        return Ok(json!({
            "auth_mode": "apikey",
            "OPENAI_API_KEY": api_key,
        }));
    }

    let tokens = account
        .tokens
        .as_ref()
        .ok_or_else(|| "OAuth account is missing tokens".to_string())?;
    if tokens.id_token.trim().is_empty() || tokens.access_token.trim().is_empty() {
        return Err("OAuth account is missing id_token/access_token".to_string());
    }

    let mut token_value = json!({
        "id_token": tokens.id_token,
        "access_token": tokens.access_token,
    });
    if let Some(refresh_token) = tokens
        .refresh_token
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        token_value["refresh_token"] = json!(refresh_token);
    }
    if let Some(account_id) = account
        .account_id
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        token_value["account_id"] = json!(account_id);
    }

    Ok(json!({
        "OPENAI_API_KEY": Value::Null,
        "tokens": token_value,
        "last_refresh": Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
    }))
}

fn write_codex_keychain(codex_home: &Path, auth_file_value: &Value) -> Result<(), String> {
    let secret = serde_json::to_string(auth_file_value)
        .map_err(|error| format!("serialize keychain payload: {error}"))?;
    let account = codex_keychain_account(codex_home);
    let output = std::process::Command::new("security")
        .arg("add-generic-password")
        .arg("-U")
        .arg("-s")
        .arg(CODEX_KEYCHAIN_SERVICE)
        .arg("-a")
        .arg(&account)
        .arg("-w")
        .arg(&secret)
        .output()
        .map_err(|error| format!("run security add-generic-password: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "write Codex keychain failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

pub fn codex_keychain_account(codex_home: &Path) -> String {
    let resolved = fs::canonicalize(codex_home).unwrap_or_else(|_| codex_home.to_path_buf());
    let mut hasher = Sha256::new();
    hasher.update(resolved.to_string_lossy().as_bytes());
    let digest = hex::encode(hasher.finalize());
    format!("cli|{}", &digest[..16])
}

fn keychain_entry_present(account: &str) -> bool {
    std::process::Command::new("security")
        .arg("find-generic-password")
        .arg("-s")
        .arg(CODEX_KEYCHAIN_SERVICE)
        .arg("-a")
        .arg(account)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn write_managed_projection(
    paths: &ResolvedPaths,
    account: &CodexAccountDetail,
) -> Result<(), String> {
    let projection = ManagedProjection {
        version: 1,
        writer: "cockpit".to_string(),
        account_id: account.id.clone(),
        email: account.email.clone(),
        token_generation: account.token_generation,
        written_at: now_timestamp(),
    };
    let content = serde_json::to_string_pretty(&projection)
        .map_err(|error| format!("serialize managed projection: {error}"))?;
    write_string_atomic(&paths.codex_home.join(CODEX_AUTH_PROJECTION_FILE), &content)
}

fn read_projection(paths: &ResolvedPaths) -> Result<Option<ManagedProjection>, String> {
    let path = paths.codex_home.join(CODEX_AUTH_PROJECTION_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let content =
        fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(|error| format!("parse {}: {error}", path.display()))
}

fn write_api_provider_to_config_toml(
    base_dir: &Path,
    account: &CodexAccountDetail,
) -> Result<(), String> {
    let config_path = base_dir.join("config.toml");
    let base_url = account
        .api_base_url
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.trim_end_matches('/').to_string());
    if !config_path.exists() && (!account.is_api_key_auth() || base_url.is_none()) {
        return Ok(());
    }

    let existing = fs::read_to_string(&config_path).unwrap_or_default();
    let mut doc = if existing.trim().is_empty() {
        toml_edit::DocumentMut::new()
    } else {
        existing
            .parse::<toml_edit::DocumentMut>()
            .map_err(|error| format!("parse config.toml: {error}"))?
    };

    if !account.is_api_key_auth() {
        let _ = doc.remove("model_provider");
        let _ = doc.remove("openai_base_url");
    } else if let Some(base_url) = base_url {
        if account
            .api_provider_mode
            .as_deref()
            .is_some_and(|mode| mode.eq_ignore_ascii_case("custom"))
        {
            let provider_id = account
                .api_provider_id
                .clone()
                .or_else(|| provider_id_from_base_url(&base_url))
                .unwrap_or_else(|| "custom_openai".to_string());
            let provider_name = account
                .api_provider_name
                .clone()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| provider_id.clone());
            let _ = doc.remove("openai_base_url");
            doc["model_provider"] = toml_edit::value(provider_id.as_str());
            if !doc.as_table().contains_key("model_providers") {
                doc["model_providers"] = toml_edit::table();
            }
            doc["model_providers"][provider_id.as_str()]["name"] = toml_edit::value(provider_name);
            doc["model_providers"][provider_id.as_str()]["base_url"] = toml_edit::value(base_url);
            doc["model_providers"][provider_id.as_str()]["wire_api"] =
                toml_edit::value("responses");
            doc["model_providers"][provider_id.as_str()]["requires_openai_auth"] =
                toml_edit::value(true);
        } else {
            let _ = doc.remove("model_provider");
            doc["openai_base_url"] = toml_edit::value(base_url);
        }
    } else {
        let _ = doc.remove("model_provider");
        let _ = doc.remove("openai_base_url");
    }

    write_string_atomic(&config_path, &doc.to_string())
}

fn provider_id_from_base_url(base_url: &str) -> Option<String> {
    let mut out = String::new();
    let mut prev_sep = false;
    for ch in base_url.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_sep = false;
        } else if !prev_sep {
            out.push('_');
            prev_sep = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn backup_before_switch(paths: &ResolvedPaths) -> Result<(), String> {
    let backup_dir = paths
        .home
        .join(".ct-codex-bridge")
        .join("backups")
        .join(Utc::now().format("%Y%m%d-%H%M%S").to_string());
    fs::create_dir_all(&backup_dir)
        .map_err(|error| format!("create backup dir {}: {error}", backup_dir.display()))?;

    copy_if_exists(
        &paths.codex_home.join("auth.json"),
        &backup_dir.join("codex-auth.json"),
    )?;
    copy_if_exists(
        &paths.codex_home.join(CODEX_AUTH_PROJECTION_FILE),
        &backup_dir.join("cockpit-codex-auth.json"),
    )?;
    copy_if_exists(
        &paths.account_index_path,
        &backup_dir.join("codex_accounts.json"),
    )?;
    Ok(())
}

fn copy_if_exists(from: &Path, to: &Path) -> Result<(), String> {
    if from.exists() {
        fs::copy(from, to)
            .map(|_| ())
            .map_err(|error| format!("backup {}: {error}", from.display()))?;
    }
    Ok(())
}

pub fn write_string_atomic(path: &Path, content: &str) -> Result<(), String> {
    write_bytes_atomic(path, content.as_bytes())
}

fn write_bytes_atomic(path: &Path, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    let parent = path
        .parent()
        .ok_or_else(|| format!("path has no parent: {}", path.display()))?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("path has no file name: {}", path.display()))?;
    let tmp_path = parent.join(format!(
        ".{file_name}.tmp.{}.{}",
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    fs::write(&tmp_path, content)
        .map_err(|error| format!("write {}: {error}", tmp_path.display()))?;
    fs::rename(&tmp_path, path).map_err(|error| {
        let _ = fs::remove_file(&tmp_path);
        format!("replace {}: {error}", path.display())
    })
}

struct RollbackSnapshot {
    files: Vec<(PathBuf, Option<Vec<u8>>)>,
    keychain_account: String,
    keychain_secret: Option<String>,
}

impl RollbackSnapshot {
    fn capture(paths: &ResolvedPaths, account_id: &str) -> Result<Self, String> {
        let files = vec![
            capture_file(paths.codex_home.join("auth.json"))?,
            capture_file(paths.codex_home.join(CODEX_AUTH_PROJECTION_FILE))?,
            capture_file(paths.codex_home.join("config.toml"))?,
            capture_file(paths.account_index_path.clone())?,
            capture_file(paths.accounts_dir.join(format!("{account_id}.json")))?,
        ];
        let keychain_account = codex_keychain_account(&paths.codex_home);
        let keychain_secret = read_keychain_secret(&keychain_account);
        Ok(Self {
            files,
            keychain_account,
            keychain_secret,
        })
    }

    fn restore(&self) -> Result<(), String> {
        for (path, content) in &self.files {
            match content {
                Some(bytes) => write_bytes_atomic(path, bytes)?,
                None => {
                    if path.exists() {
                        fs::remove_file(path).map_err(|error| {
                            format!("rollback remove {}: {error}", path.display())
                        })?;
                    }
                }
            }
        }
        restore_keychain_secret(&self.keychain_account, self.keychain_secret.as_deref());
        Ok(())
    }
}

fn capture_file(path: PathBuf) -> Result<(PathBuf, Option<Vec<u8>>), String> {
    if path.exists() {
        let content =
            fs::read(&path).map_err(|error| format!("snapshot {}: {error}", path.display()))?;
        Ok((path, Some(content)))
    } else {
        Ok((path, None))
    }
}

fn read_keychain_secret(account: &str) -> Option<String> {
    let output = std::process::Command::new("security")
        .arg("find-generic-password")
        .arg("-w")
        .arg("-s")
        .arg(CODEX_KEYCHAIN_SERVICE)
        .arg("-a")
        .arg(account)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string(),
    )
}

fn restore_keychain_secret(account: &str, secret: Option<&str>) {
    if let Some(secret) = secret {
        let _ = std::process::Command::new("security")
            .arg("add-generic-password")
            .arg("-U")
            .arg("-s")
            .arg(CODEX_KEYCHAIN_SERVICE)
            .arg("-a")
            .arg(account)
            .arg("-w")
            .arg(secret)
            .output();
    } else {
        let _ = std::process::Command::new("security")
            .arg("delete-generic-password")
            .arg("-s")
            .arg(CODEX_KEYCHAIN_SERVICE)
            .arg("-a")
            .arg(account)
            .output();
    }
}

fn read_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|key| value.get(*key))
        .find_map(|item| match item {
            Value::String(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
            Value::Number(value) => Some(value.to_string()),
            _ => None,
        })
}

fn extract_openai_api_key(value: &Value) -> Option<String> {
    value
        .get("openai_api_key")
        .or_else(|| value.get("openaiApiKey"))
        .or_else(|| value.get("OPENAI_API_KEY"))
        .and_then(|item| match item {
            Value::String(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
            _ => None,
        })
}

fn now_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn normalize_macos_app_root(path: &Path) -> Option<PathBuf> {
    let raw = path.to_string_lossy();
    raw.find(".app")
        .map(|index| PathBuf::from(&raw[..index + 4]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oauth_detail() -> CodexAccountDetail {
        CodexAccountDetail::from_value(
            json!({
                "id": "codex_abc",
                "email": "user@example.com",
                "auth_mode": "oauth",
                "account_id": "acc_42",
                "tokens": {
                    "id_token": "id.token.value",
                    "access_token": "access.token.value",
                    "refresh_token": "refresh-token"
                },
                "token_generation": 7,
                "created_at": 10,
                "last_used": 20
            }),
            None,
        )
        .unwrap()
    }

    #[test]
    fn oauth_auth_payload_matches_codex_shape_without_api_key_secret() {
        let payload = build_auth_file_value(&oauth_detail()).unwrap();
        assert!(payload.get("auth_mode").is_none());
        assert!(payload.get("OPENAI_API_KEY").unwrap().is_null());
        assert_eq!(payload["tokens"]["account_id"], "acc_42");
        assert_eq!(payload["tokens"]["refresh_token"], "refresh-token");
    }

    #[test]
    fn api_key_auth_payload_matches_codex_shape() {
        let detail = CodexAccountDetail::from_value(
            json!({
                "id": "codex_key",
                "email": "api-key@example.com",
                "auth_mode": "apikey",
                "openai_api_key": "sk-test"
            }),
            None,
        )
        .unwrap();
        let payload = build_auth_file_value(&detail).unwrap();
        assert_eq!(payload["auth_mode"], "apikey");
        assert_eq!(payload["OPENAI_API_KEY"], "sk-test");
        assert!(payload.get("tokens").is_none());
    }

    #[test]
    fn keychain_account_is_ct_compatible_hash_prefix() {
        let account = codex_keychain_account(Path::new("/Users/example/.codex"));
        assert!(account.starts_with("cli|"));
        assert_eq!(account.len(), 20);
    }

    #[test]
    fn account_view_does_not_serialize_tokens_or_api_key() {
        let view = oauth_detail().to_view(Some("codex_abc"));
        let value = serde_json::to_value(view).unwrap();
        let text = serde_json::to_string(&value).unwrap();
        assert!(!text.contains("id.token.value"));
        assert!(!text.contains("access.token.value"));
        assert!(!text.contains("refresh-token"));
        assert!(value.get("hasOAuthSnapshot").is_some());
    }
}
