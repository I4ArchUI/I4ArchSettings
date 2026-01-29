use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
pub struct ThemeInfo {
    name: String,
    path: String,
}

#[derive(Serialize)]
pub struct AppearanceState {
    pub cursor_theme: String,
    pub cursor_size: u32,
    pub gtk_theme: String,
    pub color_scheme: String,
}

fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string())
}

#[tauri::command]
pub fn get_cursor_themes() -> Vec<ThemeInfo> {
    let home = get_home_dir();
    let icons_path = Path::new(&home).join(".icons");
    let mut themes = Vec::new();

    if icons_path.exists() {
        if let Ok(entries) = fs::read_dir(icons_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let path = entry.path();
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        // Cursor themes MUST have a 'cursors' directory.
                        // Just checking index.theme is not enough as icon themes also have it.
                        if path.join("cursors").exists() {
                            themes.push(ThemeInfo {
                                name: dir_name,
                                path: path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    themes
}

#[tauri::command]
pub fn get_gtk_themes_list() -> Vec<ThemeInfo> {
    let home = get_home_dir();
    let themes_path = Path::new(&home).join(".themes");
    let mut themes = Vec::new();

    if themes_path.exists() {
        if let Ok(entries) = fs::read_dir(themes_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let path = entry.path();
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        // Basic check for gtk-3.0 or index.theme
                        if path.join("gtk-3.0").exists() || path.join("index.theme").exists() {
                            themes.push(ThemeInfo {
                                name: dir_name,
                                path: path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    themes
}

#[tauri::command]
pub fn apply_appearance_conf(
    cursor_theme: String,
    cursor_size: u32,
    gtk_theme: String,
    dark_mode: bool,
) -> Result<(), String> {
    let color_scheme = if dark_mode {
        "prefer-dark"
    } else {
        "prefer-light"
    };

    // 1. Run commands immediately
    let cmds = vec![
        vec![
            "gsettings",
            "set",
            "org.gnome.desktop.interface",
            "color-scheme",
            color_scheme,
        ],
        vec![
            "gsettings",
            "set",
            "org.gnome.desktop.interface",
            "gtk-theme",
            &gtk_theme,
        ],
        vec![
            "gsettings",
            "set",
            "org.gnome.shell.extensions.user-theme",
            "name",
            &gtk_theme,
        ],
        vec![
            "gsettings",
            "set",
            "org.gnome.desktop.interface",
            "cursor-theme",
            &cursor_theme,
        ],
    ];

    for cmd in cmds {
        let _ = Command::new(cmd[0]).args(&cmd[1..]).output();
    }

    // hyprctl setcursor
    let _ = Command::new("hyprctl")
        .arg("setcursor")
        .arg(&cursor_theme)
        .arg(cursor_size.to_string())
        .output();

    // 2. Write to theme.conf
    let home = get_home_dir();
    let config_path = Path::new(&home).join(".config/hypr/themes/theme.conf");

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let content = format!(
        "exec = gsettings set org.gnome.desktop.interface color-scheme '{}'\n\
         exec = gsettings set org.gnome.desktop.interface gtk-theme '{}'\n\
         exec = gsettings set org.gnome.shell.extensions.user-theme name '{}'\n\
         exec = gsettings set org.gnome.desktop.interface cursor-theme '{}'\n\
         exec = hyprctl setcursor {} {}\n",
        color_scheme, gtk_theme, gtk_theme, cursor_theme, cursor_theme, cursor_size
    );

    fs::write(config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}

fn run_cmd_output(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .replace("'", "")
                .to_string()
        })
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_current_appearance_config() -> AppearanceState {
    // Attempt to read from gsettings for accuracy of current state
    let cursor_theme = run_cmd_output(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "cursor-theme"],
    );
    let gtk_theme = run_cmd_output(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "gtk-theme"],
    );
    let color_scheme_raw = run_cmd_output(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "color-scheme"],
    );

    // Parse cursor size from theme.conf because system doesn't always expose it easily in a standard way across hyprland
    let mut cursor_size = 24;
    let home = get_home_dir();
    let config_path = Path::new(&home).join(".config/hypr/themes/theme.conf");

    if let Ok(content) = fs::read_to_string(config_path) {
        for line in content.lines() {
            if line.contains("hyprctl setcursor") {
                // exec = hyprctl setcursor ThemeName Size
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(size_str) = parts.last() {
                    if let Ok(s) = size_str.parse::<u32>() {
                        cursor_size = s;
                    }
                }
            }
        }
    }

    AppearanceState {
        cursor_theme: if cursor_theme.is_empty() {
            "Adwaita".to_string()
        } else {
            cursor_theme
        },
        cursor_size,
        gtk_theme: if gtk_theme.is_empty() {
            "Adwaita".to_string()
        } else {
            gtk_theme
        },
        color_scheme: color_scheme_raw,
    }
}
