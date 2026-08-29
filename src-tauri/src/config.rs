use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub position: String,      // "bottom-right", "bottom-left", "top-right", "top-left"
    pub click_through: bool,
    pub theme: String,         // "frosted-card", "minimal-hud", "dynamic-island", "cyber-neon"
    pub sensitivity: f32,      // 0.5 to 2.0 (default 1.0)
    pub auto_hide_seconds: u32,// 0 = keep standby card visible, >0 = fade completely after N seconds
    pub margin_x: i32,         // default 24
    pub margin_y: i32,         // default 24
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            position: "bottom-right".to_string(),
            click_through: false,
            theme: "frosted-card".to_string(),
            sensitivity: 1.0,
            auto_hide_seconds: 0,
            margin_x: 24,
            margin_y: 24,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("MusicMotion");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    pub fn load() -> AppConfig {
        let path = Self::get_config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    return config;
                }
            }
        }
        let default_config = AppConfig::default();
        Self::save(&default_config);
        default_config
    }

    pub fn save(config: &AppConfig) {
        let path = Self::get_config_path();
        if let Ok(json) = serde_json::to_string_pretty(config) {
            let _ = fs::write(path, json);
        }
    }

    pub fn get_themes_dir() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("MusicMotion");
        path.push("themes");
        fs::create_dir_all(&path).ok();
        path
    }
}
