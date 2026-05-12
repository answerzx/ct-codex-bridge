use ct_codex_bridge::codex::{self, ResolvedPaths, SwitchRequest};
use serde_json::json;
use std::fs;
use tempfile::TempDir;

fn write_json(path: &std::path::Path, value: serde_json::Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, serde_json::to_string_pretty(&value).unwrap()).unwrap();
}

fn fixture() -> (TempDir, ResolvedPaths) {
    let temp = TempDir::new().unwrap();
    let home = temp.path().join("home");
    let codex_home = temp.path().join("codex-home");
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&codex_home).unwrap();
    let paths = ResolvedPaths::from_home_and_codex_home(home.clone(), codex_home);

    write_json(
        &paths.account_index_path,
        json!({
            "version": "1.0",
            "current_account_id": "codex_old",
            "accounts": [
                {
                    "id": "codex_old",
                    "email": "old@example.com",
                    "plan_type": "team",
                    "created_at": 10,
                    "last_used": 20
                },
                {
                    "id": "codex_key",
                    "email": "api-key@example.com",
                    "plan_type": "API_KEY",
                    "created_at": 11,
                    "last_used": 21
                }
            ]
        }),
    );

    write_json(
        &paths.accounts_dir.join("codex_old.json"),
        json!({
            "id": "codex_old",
            "email": "old@example.com",
            "auth_mode": "oauth",
            "tokens": {
                "id_token": "old-id-token",
                "access_token": "old-access-token",
                "refresh_token": "old-refresh-token"
            },
            "created_at": 10,
            "last_used": 20
        }),
    );

    write_json(
        &paths.accounts_dir.join("codex_key.json"),
        json!({
            "id": "codex_key",
            "email": "api-key@example.com",
            "auth_mode": "apikey",
            "openai_api_key": "sk-test-portable",
            "api_base_url": "https://api.example.test/v1",
            "created_at": 11,
            "last_used": 21,
            "requires_reauth": true
        }),
    );

    (temp, paths)
}

#[test]
fn list_accounts_never_exposes_saved_credentials() {
    let (_temp, paths) = fixture();
    let response = codex::list_accounts(&paths).unwrap();
    let text = serde_json::to_string(&response).unwrap();

    assert!(text.contains("api-key@example.com"));
    assert!(!text.contains("old-id-token"));
    assert!(!text.contains("old-access-token"));
    assert!(!text.contains("old-refresh-token"));
    assert!(!text.contains("sk-test-portable"));
}

#[test]
fn switch_api_key_account_projects_auth_and_updates_current_without_restart() {
    let (_temp, paths) = fixture();
    let result = codex::switch_account(
        &paths,
        SwitchRequest {
            account_id: "codex_key".to_string(),
            restart: false,
        },
    )
    .unwrap();

    assert_eq!(result.account.id, "codex_key");
    assert!(!result.restarted);

    let index: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&paths.account_index_path).unwrap()).unwrap();
    assert_eq!(index["current_account_id"], "codex_key");

    let auth: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(paths.codex_home.join("auth.json")).unwrap())
            .unwrap();
    assert_eq!(auth["auth_mode"], "apikey");
    assert_eq!(auth["OPENAI_API_KEY"], "sk-test-portable");
    assert!(auth.get("tokens").is_none());

    let projection_path = paths.codex_home.join(".cockpit_codex_auth.json");
    let projection: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(projection_path).unwrap()).unwrap();
    assert_eq!(projection["writer"], "cockpit");
    assert_eq!(projection["account_id"], "codex_key");

    let detail: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(paths.accounts_dir.join("codex_key.json")).unwrap(),
    )
    .unwrap();
    assert!(detail["last_used"].as_i64().unwrap() > 21);
}

#[test]
fn switch_rolls_back_files_when_restart_fails() {
    let (temp, mut paths) = fixture();
    paths.codex_app_path = temp.path().join("Missing Codex.app");

    let error = codex::switch_account(
        &paths,
        SwitchRequest {
            account_id: "codex_key".to_string(),
            restart: true,
        },
    )
    .unwrap_err();

    assert!(error.contains("Codex.app not found"));

    let index: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&paths.account_index_path).unwrap()).unwrap();
    assert_eq!(index["current_account_id"], "codex_old");
    assert!(!paths.codex_home.join("auth.json").exists());
    assert!(!paths.codex_home.join(".cockpit_codex_auth.json").exists());

    let detail: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(paths.accounts_dir.join("codex_key.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(detail["last_used"], 21);
}
