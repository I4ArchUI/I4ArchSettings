use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
pub struct AppItem {
    name: String,
    icon: String,
    description: String,
    desktop_file: String,
    full_path: String,
}

#[derive(Serialize)]
pub struct PackageItem {
    name: String,
    version: String,
}

/// Lists installed desktop applications by scanning /usr/share/applications
#[tauri::command]
pub fn get_installed_apps() -> Vec<AppItem> {
    let mut apps = Vec::new();
    let paths = ["/usr/share/applications", "/usr/local/share/applications"];

    // checks ~/.local/share/applications as well
    let home = std::env::var("HOME").unwrap_or_default();
    let local_apps = format!("{}/.local/share/applications", home);

    let mut all_paths = paths.to_vec();
    if !home.is_empty() {
        all_paths.push(&local_apps);
    }

    for path in all_paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Some(app) = parse_desktop_file(&path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    // Sort by name
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn parse_desktop_file(path: &Path) -> Option<AppItem> {
    let content = fs::read_to_string(path).ok()?;
    let mut name = String::new();
    let mut icon = String::new();
    let mut comment = String::new();
    let mut no_display = false;
    let mut type_app = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Name=") && name.is_empty() {
            name = trimmed.trim_start_matches("Name=").to_string();
        } else if trimmed.starts_with("Icon=") && icon.is_empty() {
            icon = trimmed.trim_start_matches("Icon=").to_string();
        } else if trimmed.starts_with("Comment=") && comment.is_empty() {
            comment = trimmed.trim_start_matches("Comment=").to_string();
        } else if trimmed == "NoDisplay=true" {
            no_display = true;
        } else if trimmed == "Type=Application" {
            type_app = true;
        }
    }

    if name.is_empty() || no_display || !type_app {
        return None;
    }

    Some(AppItem {
        name,
        icon,
        description: comment,
        desktop_file: path.file_name()?.to_string_lossy().to_string(),
        full_path: path.to_string_lossy().to_string(),
    })
}

/// Lists all installed packages using pacman
#[tauri::command]
pub fn get_installed_packages() -> Vec<PackageItem> {
    let mut packages = Vec::new();
    if let Ok(output) = Command::new("pacman").arg("-Q").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            // Format: name version
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(PackageItem {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                });
            }
        }
    }
    packages
}

/// Uninstalls a package or app (by package name)
#[tauri::command]
pub fn uninstall_package(name: String) -> Result<(), String> {
    // List of terminals to try, in order of preference
    let terminals = [
        "kitty",
        "alacritty",
        "wezterm",
        "gnome-terminal",
        "konsole",
        "xfce4-terminal",
    ];
    // The command to execute: sudo pacman -Rns <name>
    let uninstall_cmd = format!("sudo pacman -R {}", name);

    for term in terminals {
        let mut cmd = Command::new(term);

        match term {
            "gnome-terminal" | "xfce4-terminal" => {
                cmd.args(&[
                    "--",
                    "bash",
                    "-c",
                    &format!(
                        "{}; echo; read -p 'Press Enter to close...' -n 1",
                        uninstall_cmd
                    ),
                ]);
            }
            "konsole" => {
                cmd.args(&[
                    "-e",
                    "bash",
                    "-c",
                    &format!(
                        "{}; echo; read -p 'Press Enter to close...' -n 1",
                        uninstall_cmd
                    ),
                ]);
            }
            "kitty" | "alacritty" | "wezterm" => {
                cmd.args(&[
                    "-e",
                    "sh",
                    "-c",
                    &format!(
                        "{}; echo; read -p 'Press Enter to close...' -n 1",
                        uninstall_cmd
                    ),
                ]);
            }
            _ => continue,
        }

        if let Ok(mut child) = cmd.spawn() {
            let _ = child.wait();
            return Ok(());
        }
    }

    Err("No supported terminal emulator found".to_string())
}

/// Uninstalls an app by resolving its package owner from the desktop file path
#[tauri::command]
pub fn uninstall_app(full_path: String) -> Result<(), String> {
    // Resolve package name from file owner
    // pacman -Qo <path>
    // Output: /path/to/file is owned by <package> <version>

    let output = Command::new("pacman")
        .arg("-Qo")
        .arg(&full_path)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Failed to resolve package owner for this app".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Example: /usr/share/applications/vlc.desktop is owned by vlc 3.0.20-7

    // Split by " is owned by "
    let parts: Vec<&str> = stdout.split(" is owned by ").collect();
    if parts.len() < 2 {
        return Err("Could not parse package owner".to_string());
    }

    // The second part is "<package> <version>\n"
    let pkg_info = parts[1].trim();
    let pkg_parts: Vec<&str> = pkg_info.split_whitespace().collect();
    if pkg_parts.is_empty() {
        return Err("Could not parse package name".to_string());
    }

    let package_name = pkg_parts[0];

    // Call uninstall_package with the resolved name
    uninstall_package(package_name.to_string())
}
