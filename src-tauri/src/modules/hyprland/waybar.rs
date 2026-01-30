use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Changes the Waybar position by copying configuration files from a specific theme directory.
#[tauri::command]
pub fn set_waybar_position(position: String) -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|_| "Cannot get HOME directory".to_string())?;
    let waybar_config_dir = PathBuf::from(&home).join(".config/waybar");
    let position_dir = waybar_config_dir.join("themes").join(&position);

    // Verify the target position directory exists
    if !position_dir.exists() {
        return Err(format!("Position directory '{}' does not exist", position));
    }

    // Deploy config.jsonc
    let config_src = position_dir.join("config.jsonc");
    let config_dest = waybar_config_dir.join("config.jsonc");

    if config_src.exists() {
        fs::copy(&config_src, &config_dest)
            .map_err(|e| format!("Failed to copy config.jsonc: {}", e))?;
    } else {
        return Err(format!("config.jsonc not found in {}", position));
    }

    // Deploy style.css
    let style_src = position_dir.join("style.css");
    let style_dest = waybar_config_dir.join("style.css");

    if style_src.exists() {
        fs::copy(&style_src, &style_dest)
            .map_err(|e| format!("Failed to copy style.css: {}", e))?;
    } else {
        return Err(format!("style.css not found in {}", position));
    }

    reload_waybar()?;

    Ok(format!("Waybar position changed to {}", position))
}

/// Restarts the Waybar process to apply configuration changes.
fn reload_waybar() -> Result<(), String> {
    let _ = Command::new("pkill").arg("waybar").output();
    std::thread::sleep(std::time::Duration::from_millis(300));
    Command::new("hyprctl")
        .arg("dispatch")
        .arg("exec")
        .arg("waybar")
        .spawn()
        .map_err(|e| format!("Failed to start waybar: {}", e))?;
    Ok(())
}

/// Parses the current Waybar configuration to determine its active screen position.
#[tauri::command]
pub fn get_waybar_position() -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|_| "Cannot get HOME directory".to_string())?;
    let waybar_config_dir = PathBuf::from(&home).join(".config/waybar");
    let config_file = waybar_config_dir.join("config.jsonc");

    if !config_file.exists() {
        return Ok("top".to_string());
    }

    let content =
        fs::read_to_string(&config_file).map_err(|e| format!("Failed to read config: {}", e))?;

    // Search for the "position" field in the configuration
    for line in content.lines() {
        if line.contains("\"position\"") {
            if line.contains("\"top\"") {
                return Ok("top".to_string());
            } else if line.contains("\"bottom\"") {
                return Ok("bottom".to_string());
            } else if line.contains("\"left\"") {
                return Ok("left".to_string());
            } else if line.contains("\"right\"") {
                return Ok("right".to_string());
            }
        }
    }

    Ok("top".to_string())
}
