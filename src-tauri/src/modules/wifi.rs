use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

/// Represents a Wi-Fi network with its signal strength and status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    ssid: String,
    security: String,
    bars: String,
    signal: u8,
    active: bool,
}

/// Configuration for a Wi-Fi connection (IPv4 settings).
#[derive(Debug, Serialize, Deserialize)]
pub struct WifiConfig {
    method: String,
    ip_address: String,
    prefix: i32,
    gateway: String,
    dns: String,
}

/// Checks if Wi-Fi radio is currently enabled.
#[tauri::command]
pub async fn get_wifi_status() -> bool {
    let output = Command::new("nmcli")
        .args(&["radio", "wifi"])
        .output()
        .await;

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.trim() == "enabled"
        }
        Err(_) => false,
    }
}

/// Toggles Wi-Fi radio power state.
#[tauri::command]
pub async fn toggle_wifi(enable: bool) -> Result<(), String> {
    let state = if enable { "on" } else { "off" };
    Command::new("nmcli")
        .args(&["radio", "wifi", state])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Scans for available Wi-Fi networks and returns a deduplicated list.
#[tauri::command]
pub async fn scan_wifi() -> Result<Vec<WifiNetwork>, String> {
    // nmcli -t -f ACTIVE,SSID,SECURITY,BARS,SIGNAL dev wifi list
    let output = Command::new("nmcli")
        .args(&[
            "-t",
            "-f",
            "ACTIVE,SSID,SECURITY,BARS,SIGNAL",
            "dev",
            "wifi",
            "list",
        ])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map: HashMap<String, WifiNetwork> = HashMap::new();

    for line in stdout.lines() {
        // Parse colon-separated output carefully (SSID may contain colons)
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() >= 5 {
            let active = parts[0] == "yes";
            let signal: u8 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);
            let bars = parts[parts.len() - 2].to_string();
            let security = parts[parts.len() - 3].to_string();

            // SSID is between the 'active' field and the 'security' field
            let ssid_parts = &parts[1..parts.len() - 3];
            let ssid = ssid_parts.join(":");

            if !ssid.is_empty() {
                let network = WifiNetwork {
                    ssid: ssid.clone(),
                    security,
                    bars,
                    signal,
                    active,
                };

                // Deduplicate by SSID, preferring the active connection or the strongest signal
                match map.get(&ssid) {
                    Some(existing) => {
                        let is_better = if network.active && !existing.active {
                            true
                        } else if network.active == existing.active {
                            network.signal > existing.signal
                        } else {
                            false
                        };

                        if is_better {
                            map.insert(ssid, network);
                        }
                    }
                    None => {
                        map.insert(ssid, network);
                    }
                }
            }
        }
    }

    let mut networks: Vec<WifiNetwork> = map.into_values().collect();
    // Sort: Active first, then Signal strength descending
    networks.sort_by(|a, b| b.active.cmp(&a.active).then(b.signal.cmp(&a.signal)));

    Ok(networks)
}

/// Connects to a Wi-Fi network using the provided SSID and optional password.
#[tauri::command]
pub async fn connect_wifi(ssid: String, password: Option<String>) -> Result<String, String> {
    let mut cmd = Command::new("nmcli");
    cmd.args(&["dev", "wifi", "connect", &ssid]);

    if let Some(pwd) = password {
        cmd.args(&["password", &pwd]);
    }

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Connected successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Retrieves the connection configuration (IPv4) for a given SSID.
#[tauri::command]
pub async fn get_wifi_config(ssid: String) -> Result<WifiConfig, String> {
    // Fetch persistent configuration and current active values from nmcli
    let output = Command::new("nmcli")
        .args(&[
            "-g",
            "ipv4.method,ipv4.addresses,ipv4.gateway,ipv4.dns,IP4.ADDRESS,IP4.GATEWAY,IP4.DNS",
            "connection",
            "show",
            &ssid,
        ])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        // Return default DHCP settings if no profile exists
        return Ok(WifiConfig {
            method: "auto".to_string(),
            ip_address: "".to_string(),
            prefix: 24,
            gateway: "".to_string(),
            dns: "".to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Map output lines to specific fields
    if lines.len() < 4 {
        return Ok(WifiConfig {
            method: "auto".to_string(),
            ip_address: "".to_string(),
            prefix: 24,
            gateway: "".to_string(),
            dns: "".to_string(),
        });
    }

    let conf_method = lines.get(0).unwrap_or(&"");
    let conf_addr = lines.get(1).unwrap_or(&"");
    let conf_gw = lines.get(2).unwrap_or(&"");
    let conf_dns = lines.get(3).unwrap_or(&"");

    let active_addr = lines.get(4).unwrap_or(&"");
    let active_gw = lines.get(5).unwrap_or(&"");
    let active_dns = lines.get(6).unwrap_or(&"");

    let method = if conf_method.is_empty() {
        "auto".to_string()
    } else {
        conf_method.to_string()
    };

    // Resolve IP address and prefix (preferring persistent config over active values)
    let full_address = if !conf_addr.is_empty() {
        conf_addr.to_string()
    } else if !active_addr.is_empty() {
        active_addr.to_string()
    } else {
        String::new()
    };

    let (ip_address, prefix) = if let Some((ip, p)) = full_address.split_once('/') {
        (ip.to_string(), p.parse().unwrap_or(24))
    } else {
        (full_address, 24)
    };

    let gateway = if !conf_gw.is_empty() {
        conf_gw.to_string()
    } else if !active_gw.is_empty() {
        active_gw.to_string()
    } else {
        String::new()
    };

    let dns = if !conf_dns.is_empty() {
        conf_dns.to_string()
    } else if !active_dns.is_empty() {
        active_dns.to_string()
    } else {
        String::new()
    };

    Ok(WifiConfig {
        method,
        ip_address,
        prefix,
        gateway,
        dns,
    })
}

/// Updates the connection configuration for a given SSID and applies changes.
#[tauri::command]
pub async fn set_wifi_config(ssid: String, config: WifiConfig) -> Result<(), String> {
    let mut args = vec!["connection".to_string(), "modify".to_string(), ssid.clone()];

    args.push("ipv4.method".to_string());
    args.push(config.method.clone());

    if config.method == "manual" {
        let full_address = format!("{}/{}", config.ip_address, config.prefix);
        args.push("ipv4.addresses".to_string());
        args.push(full_address);

        args.push("ipv4.gateway".to_string());
        args.push(if !config.gateway.is_empty() {
            config.gateway.clone()
        } else {
            "".to_string()
        });
    } else {
        // Clear static settings when switching back to auto
        args.push("ipv4.addresses".to_string());
        args.push("".to_string());
        args.push("ipv4.gateway".to_string());
        args.push("".to_string());
    }

    // Configure DNS settings and handle 'ignore-auto-dns'
    args.push("ipv4.dns".to_string());
    if !config.dns.is_empty() {
        args.push(config.dns.clone());
        args.push("ipv4.ignore-auto-dns".to_string());
        args.push("yes".to_string());
    } else {
        args.push("".to_string());
        args.push("ipv4.ignore-auto-dns".to_string());
        args.push("no".to_string());
    }

    // Apply configuration changes
    let output = Command::new("nmcli")
        .args(&args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Failed to configure: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Re-activate connection to apply settings immediately
    let output = Command::new("nmcli")
        .args(&["connection", "up", &ssid])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Configuration saved but activation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
