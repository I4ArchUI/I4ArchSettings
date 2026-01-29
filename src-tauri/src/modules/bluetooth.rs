use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct BluetoothState {
    pub scan_process: Mutex<Option<Child>>,
}

impl BluetoothState {
    pub fn new() -> Self {
        Self {
            scan_process: Mutex::new(None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BluetoothDevice {
    mac: String,
    name: String,
    connected: bool,
    paired: bool,
}

#[tauri::command]
pub async fn get_bluetooth_status() -> bool {
    let output = Command::new("bluetoothctl").args(&["show"]).output().await;

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("Powered: yes")
        }
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn toggle_bluetooth(enable: bool) -> Result<(), String> {
    let state = if enable { "on" } else { "off" };
    Command::new("bluetoothctl")
        .args(&["power", state])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

use tokio::io::AsyncWriteExt;

#[tauri::command]
pub async fn start_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let mut process_guard = state.scan_process.lock().await;

    if process_guard.is_some() {
        return Ok(());
    }

    // Spawn bluetoothctl in interactive mode
    let mut child = Command::new("bluetoothctl")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| e.to_string())?;

    // Get handle to stdin
    if let Some(mut stdin) = child.stdin.take() {
        // Send commands to interactive shell
        // We ignore errors here because if it fails, the child likely died,
        // which will be handled/ignored until next start_scan attempt.
        let _ = stdin.write_all(b"power on\n").await;
        let _ = stdin.write_all(b"scan on\n").await;

        // We drop stdin here, but the process stays running until killed?
        // Actually, for some commands, closing stdin might exit the shell.
        // bluetoothctl interactive mode usually exits on EOF.
        // We MUST NOT close stdin if we want it to stay running!

        // To keep stdin open, we need to store it in the state OR just keep the child
        // running and *maybe* it stays alive if we don't explicitly close stdin?
        // In Rust, `child.stdin` is Option<ChildStdin>. taking it moves it out of child.
        // When `stdin` variable goes out of scope, it is dropped, which Closes the pipe.
        // Closing stdin causes bluetoothctl to exit.

        // Solution: We cannot just fire and forget into stdin.
        // We need to keep stdin alive or loop it.
        // Because `start_scan` returns, we can't await a loop.
        // We have to store stdin in the state too? Or spawn a task to hold it?

        // Easier: Spawn a tokio task that holds stdin and keeps it open (maybe sending keepalives or just sleeping)
        tokio::spawn(async move {
            // Keep stdin open indefinitely
            loop {
                sleep(Duration::from_secs(60)).await;
                // Optional: send a no-op or re-issue scan on just in case?
                if let Err(_) = stdin.write_all(b"scan on\n").await {
                    break;
                }
            }
        });
    }

    *process_guard = Some(child);
    Ok(())
}

#[tauri::command]
pub async fn stop_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let mut process_guard = state.scan_process.lock().await;

    if let Some(mut child) = process_guard.take() {
        let _ = child.kill().await;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_bluetooth_devices() -> Result<Vec<BluetoothDevice>, String> {
    // List available devices (BlueZ DB)
    let output = Command::new("bluetoothctl")
        .args(&["devices"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    for line in stdout.lines() {
        // Format: Device XX:XX:XX:XX:XX:XX Name...
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "Device" {
            let mac = parts[1].to_string();
            let name = parts[2..].join(" ");

            // Filtering junk:
            // 1. If name is exactly proper MAC (XX:XX...) - keep (rare but valid?)
            // 2. If name is dashed MAC (XX-XX...) - clearly generated junk alias
            // 3. If name is empty - junk

            let normalized_name = name.replace("-", ":");
            let is_mac_alias = normalized_name.eq_ignore_ascii_case(&mac);

            // Only skip if it looks like a junk alias AND we aren't connected?
            // For now, assume if it's a MAC-alias, it's junk/unpaired.
            // Real devices usually have real names.
            if is_mac_alias {
                continue;
            }

            devices.push(BluetoothDevice {
                mac,
                name,
                connected: false,
                paired: true,
            });
        }
    }

    Ok(devices)
}

#[tauri::command]
pub async fn connect_bluetooth(mac: String) -> Result<String, String> {
    let output = Command::new("bluetoothctl")
        .args(&["connect", &mac])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("Connection successful") {
        Ok("Connected".to_string())
    } else {
        Err(stdout.to_string())
    }
}
