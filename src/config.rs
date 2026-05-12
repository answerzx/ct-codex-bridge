use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_secret: Option<String>,
}

impl BridgeConfig {
    pub fn load_or_default() -> Result<Self, String> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content =
            std::fs::read_to_string(&path).map_err(|error| format!("read config: {error}"))?;
        if content.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(&content).map_err(|error| format!("parse config: {error}"))
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("create config dir: {error}"))?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|error| format!("serialize config: {error}"))?;
        crate::codex::write_string_atomic(&path, &content)
    }
}

pub fn config_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "unable to resolve home directory".to_string())?;
    Ok(home.join(".ct-codex-bridge"))
}

pub fn config_path() -> Result<PathBuf, String> {
    Ok(config_dir()?.join("config.json"))
}
