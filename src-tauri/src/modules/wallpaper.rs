use std::process::Command;

#[tauri::command]
pub fn set_wallpaper(file_path: String) -> Result<(), String> {
    if file_path.is_empty() {
        return Err("File path is empty".to_string());
    }

    // Prepare the script
    // Using sh -c to allow ~ expansion for destination and swww command execution
    let script = format!(
        r#"
        mkdir -p ~/.config/hypr/themes
        rm -f ~/.config/hypr/themes/background.png
        cp "{}" ~/.config/hypr/themes/background.png
        swww img ~/.config/hypr/themes/background.png --transition-fps 60 --transition-step 255 --transition-type any
        "#,
        file_path
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn get_current_wallpaper_path() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    format!("{}/.config/hypr/themes/background.png", home)
}

#[tauri::command]
pub fn get_wallpaper_base64() -> Result<String, String> {
    use base64::{engine::general_purpose, Engine as _};
    use std::fs;

    let home = std::env::var("HOME").unwrap_or_default();
    let path = format!("{}/.config/hypr/themes/background.png", home);

    match fs::read(&path) {
        Ok(bytes) => Ok(general_purpose::STANDARD.encode(&bytes)),
        Err(e) => Err(e.to_string()),
    }
}
