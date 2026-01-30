use serde::{Deserialize, Serialize};
use std::process::Command;

/// Represents an active workspace on a monitor.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveWorkspace {
    pub id: i32,
    pub name: String,
}

/// Detailed information about a connected monitor.
#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub transform: i32,
    pub focused: bool,
    #[serde(rename = "activeWorkspace")]
    pub active_workspace: ActiveWorkspace,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub mirror: Option<String>,
}

/// Default value for monitor enabled state.
fn default_enabled() -> bool {
    true
}

/// Retrieves the current monitor configuration from Hyprland.
#[tauri::command]
pub fn get_displays() -> Result<Vec<Monitor>, String> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let monitors: Vec<Monitor> =
        serde_json::from_str(&output_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(monitors)
}

/// Returns the path to the Hyprland monitors configuration file.
fn get_hyprland_monitors_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let config_dir = std::path::PathBuf::from(home)
        .join(".config")
        .join("hypr")
        .join("configs");

    if !config_dir.exists() {
        let _ = std::fs::create_dir_all(&config_dir);
    }

    config_dir.join("monitors.conf")
}

/// Saves the provided monitor configuration to the Hyprland config file.
#[tauri::command]
pub fn save_displays(monitors: Vec<Monitor>) -> Result<(), String> {
    // Generate Hyprland configuration lines for each monitor
    let config_lines: Vec<String> = monitors
        .iter()
        .map(|m| {
            if !m.enabled {
                format!("monitor={},disable", m.name)
            } else if let Some(ref mirror_source) = m.mirror {
                format!(
                    "monitor={},{:.0}x{:.0}@{:.3},{:.0}x{:.0},{:.1},transform,{},mirror,{}",
                    m.name,
                    m.width,
                    m.height,
                    m.refresh_rate,
                    m.x,
                    m.y,
                    m.scale,
                    m.transform,
                    mirror_source
                )
            } else {
                format!(
                    "monitor={},{:.0}x{:.0}@{:.3},{:.0}x{:.0},{:.1},transform,{}",
                    m.name, m.width, m.height, m.refresh_rate, m.x, m.y, m.scale, m.transform
                )
            }
        })
        .collect();

    let config_content = config_lines.join("\n");
    let path = get_hyprland_monitors_path();

    // Perform an atomic write using a temporary file to avoid configuration corruption
    let temp_path = path.with_extension("tmp");
    std::fs::write(&temp_path, &config_content)
        .map_err(|e| format!("Failed to write temp config: {}", e))?;

    std::fs::rename(&temp_path, &path).map_err(|e| format!("Failed to finalize config: {}", e))?;

    Ok(())
}
