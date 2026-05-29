use std::process::Command;

/// Sets the desktop wallpaper by copying the file to a standard location and using swww.
#[tauri::command]
pub fn set_wallpaper(file_path: String) -> Result<(), String> {
    if file_path.is_empty() {
        return Err("File path is empty".to_string());
    }

    // Execute script to copy the wallpaper and update swww
    let script = format!(
        r#"
        mkdir -p ~/.config/hypr/themes
        rm -f ~/.config/hypr/themes/background.png
        cp "{}" ~/.config/hypr/themes/background.png
        awww img ~/.config/hypr/themes/background.png --transition-fps 60 --transition-step 255 --transition-type any
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

/// Returns the standard path where the current wallpaper is stored.
#[tauri::command]
pub fn get_current_wallpaper_path() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    format!("{}/.config/hypr/themes/background.png", home)
}

/// Reads the current wallpaper file and returns its content as a Base64 encoded string.
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

/// Lists all image files inside a specified directory.
#[tauri::command]
pub fn list_wallpapers_in_dir(dir_path: String) -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::Path;

    let mut resolved_path = dir_path.clone();
    if resolved_path.starts_with('~') {
        let home = std::env::var("HOME").unwrap_or_default();
        resolved_path = resolved_path.replacen('~', &home, 1);
    }

    let path = Path::new(&resolved_path);
    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ext_str == "png" || ext_str == "jpg" || ext_str == "jpeg" || ext_str == "webp" {
                        images.push(file_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    images.sort();
    Ok(images)
}

/// Downloads a wallpaper from a URL and sets it as the active desktop wallpaper.
#[tauri::command]
pub fn download_and_set_wallpaper(url: String) -> Result<(), String> {
    if url.is_empty() {
        return Err("URL is empty".to_string());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    let dest_path = format!("{}/.config/hypr/themes/downloaded_wallpaper.png", home);

    // Download using curl
    let script = format!(
        r#"
        mkdir -p ~/.config/hypr/themes
        curl -L -s -o "{}" "{}"
        "#,
        dest_path, url
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    set_wallpaper(dest_path)
}


