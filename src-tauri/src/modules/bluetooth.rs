use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::time::{sleep, Duration};

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

#[tauri::command]
pub async fn scan_bluetooth() -> Result<Vec<BluetoothDevice>, String> {
    // Attempt to start scanning in background (fire and forgetish, or brief wait)
    // We don't want to block for 3s. We just want to ensure scanning *is* active.
    // Ideally, the frontend calls a 'start_scan' once.
    // Here we will try to enable scan if not enabled, but not wait long.

    // Check if scanning is on? bluetoothctl show contains "Discovering: yes"

    // Run scan for a short duration to populate devices
    let _ = Command::new("timeout")
        .args(&["2s", "bluetoothctl", "scan", "on"])
        .output()
        .await;

    // Wait a tiny bit for devices to populate if this was first run?
    sleep(Duration::from_millis(200)).await;

    // Now list devices
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

            // basic check if likely connected (often in 'info')
            // For optimization, we skip deep info check.
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
