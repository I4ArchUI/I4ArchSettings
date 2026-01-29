use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string())
}

#[tauri::command]
pub fn get_startup_commands() -> Vec<String> {
    let home = get_home_dir();
    let config_path = Path::new(&home).join(".config/hypr/exec.conf");
    let mut commands = Vec::new();

    if let Ok(content) = fs::read_to_string(config_path) {
        for line in content.lines() {
            let trim = line.trim();
            if trim.starts_with("exec-once") {
                if let Some((_, cmd)) = trim.split_once('=') {
                    commands.push(cmd.trim().to_string());
                }
            }
        }
    }
    commands
}

#[tauri::command]
pub fn save_startup_commands(commands: Vec<String>) -> Result<(), String> {
    let home = get_home_dir();
    let config_path = Path::new(&home).join(".config/hypr/exec.conf");

    // Ensure directory exists
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut content = String::new();
    for cmd in commands {
        if !cmd.trim().is_empty() {
            content.push_str(&format!("exec-once = {}\n", cmd.trim()));
        }
    }

    fs::write(config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}
