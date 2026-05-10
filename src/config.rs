use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub vault_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct TomlConfig {
    vault: Option<Config>,
}

impl Config {
    pub fn load() -> Self {
        let mut merged = Self::default();

        if let Some(path) = Self::path() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(toml) = toml::from_str::<TomlConfig>(&content) {
                    if let Some(v) = toml.vault {
                        merged.vault_path = v.vault_path.or(merged.vault_path);
                    }
                }
            }
        }

        if let Ok(v) = env::var("CLAUDE_VAULT_PATH") {
            merged.vault_path = Some(PathBuf::from(v));
        }

        if merged.vault_path.is_none() {
            let default = dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("claude-vault")
                .join("vault.db");
            merged.vault_path = Some(default);
        }

        merged
    }

    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("claude-vault").join("config.toml"))
    }
}
