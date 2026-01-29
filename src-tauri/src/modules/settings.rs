use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub theme: String,
    #[serde(default = "default_waybar_position")]
    pub waybar_position: String,
}

fn default_waybar_position() -> String {
    "top".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            waybar_position: "top".to_string(),
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let config_dir = PathBuf::from(home).join(".config").join("i4archsettings");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    config_dir.join("settings.json")
}

#[tauri::command]
pub fn get_app_settings() -> AppSettings {
    let path = get_config_path();

    if !path.exists() {
        return AppSettings::default();
    }

    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

#[tauri::command]
pub fn save_app_settings(settings: AppSettings) -> Result<(), String> {
    // Apply system theme change using gsettings
    let scheme = if settings.theme == "dark" {
        "prefer-dark"
    } else {
        "default"
    };

    // Fire and forget the command, checking status isn't strictly necessary for the save to succeed
    // but we'll log if it fails in a real app. Here we just try to run it.
    let _ = std::process::Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output();

    let path = get_config_path();
    match serde_json::to_string_pretty(&settings) {
        Ok(json) => fs::write(path, json).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
