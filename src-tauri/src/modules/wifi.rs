use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    ssid: String,
    security: String,
    bars: String,
    signal: u8,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WifiConfig {
    method: String,
    ip_address: String,
    prefix: i32,
    gateway: String,
    dns: String,
}

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
        // Format: ACTIVE:SSID:SECURITY:BARS:SIGNAL
        // Note: SSID can contain colons, need to handle split carefully or rejoin.
        // But since we asked for specific fields, we know the structure.
        // Active is first, Signal is last, Bars is second to last.

        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() >= 5 {
            let active = parts[0] == "yes";
            let signal: u8 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);
            let bars = parts[parts.len() - 2].to_string();
            let security = parts[parts.len() - 3].to_string();

            // SSID is everything between index 1 and len-3
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

                // Deduplication logic: Insert if not exists, or replace if better.
                // Better means: currently active OR (same active state AND higher signal)
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
    // Sort by Active first, then Signal strength descending
    networks.sort_by(|a, b| b.active.cmp(&a.active).then(b.signal.cmp(&a.signal)));

    Ok(networks)
}

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

#[tauri::command]
pub async fn get_wifi_config(ssid: String) -> Result<WifiConfig, String> {
    // We fetch both the persistent configuration (ipv4.*) and the current active values (IP4.*)
    // accessible via 'connection show' (active values present if connected).
    // Using -g returns just the values, one per line (or separated by newlines).
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
        // Likely no connection profile exists, return default DHCP
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

    // We requested 7 fields.
    // 0: ipv4.method
    // 1: ipv4.addresses
    // 2: ipv4.gateway
    // 3: ipv4.dns
    // 4: IP4.ADDRESS (active)
    // 5: IP4.GATEWAY (active)
    // 6: IP4.DNS (active)

    if lines.len() < 4 {
        // Fallback if parsing failed or fields missing
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

    // Address/Prefix: Use config if present, else active
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

    // Gateway
    let gateway = if !conf_gw.is_empty() {
        conf_gw.to_string()
    } else if !active_gw.is_empty() {
        active_gw.to_string()
    } else {
        String::new()
    };

    // DNS
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

#[tauri::command]
pub async fn set_wifi_config(ssid: String, config: WifiConfig) -> Result<(), String> {
    // Construct single atomic modify command
    let mut args = vec!["connection".to_string(), "modify".to_string(), ssid.clone()];

    args.push("ipv4.method".to_string());
    args.push(config.method.clone());

    if config.method == "manual" {
        let full_address = format!("{}/{}", config.ip_address, config.prefix);
        args.push("ipv4.addresses".to_string());
        args.push(full_address);

        args.push("ipv4.gateway".to_string());
        if !config.gateway.is_empty() {
            args.push(config.gateway.clone());
        } else {
            args.push("".to_string());
        }
    } else {
        // Clean up manual settings when switching to auto (dhcp)
        // We do NOT clear DNS here anymore, as we handle it below.
        args.push("ipv4.addresses".to_string());
        args.push("".to_string());
        args.push("ipv4.gateway".to_string());
        args.push("".to_string());
    }

    // Handle DNS separately (for both Auto and Manual)
    args.push("ipv4.dns".to_string());
    if !config.dns.is_empty() {
        args.push(config.dns.clone());
        // If we set custom DNS, we usually want to ignore auto DNS to ensure our DNS is used/preferred
        args.push("ipv4.ignore-auto-dns".to_string());
        args.push("yes".to_string());
    } else {
        args.push("".to_string());
        // Restore auto DNS behavior
        args.push("ipv4.ignore-auto-dns".to_string());
        args.push("no".to_string());
    }

    // 1. Execute Modify
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

    // 2. Re-activate connection (Up) to apply changes immediately
    let output = Command::new("nmcli")
        .args(&["connection", "up", &ssid])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!("Configuration saved but activation failed. You may need to reconnect manually. Error: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}
