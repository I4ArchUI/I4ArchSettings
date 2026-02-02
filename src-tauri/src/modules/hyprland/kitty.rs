use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Sets the Kitty terminal theme by copying the configuration file and reloading Kitty.
#[tauri::command]
pub fn set_kitty_theme(theme: String) -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|_| "Cannot get HOME directory".to_string())?;
    let kitty_config_dir = PathBuf::from(&home).join(".config/kitty");
    // Path: ~/.config/kitty/themes/{theme}/kitty.conf
    let theme_path = kitty_config_dir
        .join("themes")
        .join(&theme)
        .join("kitty.conf");
    let config_dest = kitty_config_dir.join("kitty.conf");

    if theme_path.exists() {
        fs::copy(&theme_path, &config_dest)
            .map_err(|e| format!("Failed to copy kitty.conf: {}", e))?;
        let _ = Command::new("pkill").arg("-USR1").arg("kitty").output();

        Ok(format!("Kitty theme set to {}", theme))
    } else {
        Err(format!("Kitty theme file not found at {:?}", theme_path))
    }
}
