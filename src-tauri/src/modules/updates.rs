use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct UpdatePackage {
    name: String,
    old_version: String,
    new_version: String,
}

#[tauri::command]
pub fn check_updates() -> Vec<UpdatePackage> {
    let mut updates = Vec::new();

    // use checkupdates from pacman-contrib
    if let Ok(output) = Command::new("checkupdates").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            // Output format: package old_version -> new_version
            // Example: alsa-card-profiles 1:1.0.7-1 -> 1:1.0.8-1
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                updates.push(UpdatePackage {
                    name: parts[0].to_string(),
                    old_version: parts[1].to_string(),
                    new_version: parts[3].to_string(),
                });
            }
        }
    }

    updates
}

#[tauri::command]
pub fn update_system() -> Result<(), String> {
    // List of terminals to try, in order of preference
    let terminals = [
        "kitty",
        "alacritty",
        "wezterm",
        "gnome-terminal",
        "konsole",
        "xfce4-terminal",
    ];
    // The command to execute
    let update_cmd = "sudo pacman -Syu";

    for term in terminals {
        let mut cmd = Command::new(term);

        // Configuration for different terminals
        match term {
            "gnome-terminal" | "xfce4-terminal" => {
                // These often require -- to separate terminal args from command
                // and keeping the window open requires a read or shell wrapper
                cmd.args(&[
                    "--",
                    "bash",
                    "-c",
                    &format!(
                        "{}; echo; read -p 'Press Enter to close...' -n 1",
                        update_cmd
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
                        update_cmd
                    ),
                ]);
            }
            "kitty" | "alacritty" | "wezterm" => {
                // These usually support -e 'command'
                // We wrap in sh -c to allow multiple commands (update + pause)
                cmd.args(&[
                    "-e",
                    "sh",
                    "-c",
                    &format!(
                        "{}; echo; read -p 'Press Enter to close...' -n 1",
                        update_cmd
                    ),
                ]);
            }
            _ => continue,
        }

        // Attempt to spawn the terminal
        // We use spawn() because we don't need to wait for it to finish, just launch it
        if let Ok(mut child) = cmd.spawn() {
            let _ = child.wait();
            return Ok(());
        }
    }

    Err("No supported terminal emulator found (kitty, alacritty, wezterm, gnome-terminal, konsole, xfce4-terminal)".to_string())
}
