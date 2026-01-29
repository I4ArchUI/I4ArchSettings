use serde::{Deserialize, Serialize};
use tokio::process::Command;

/// Represents a VPN connection with its metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VpnConnection {
    pub uuid: String,
    pub name: String,
    pub active: bool,
    pub type_name: String, // e.g. "wireguard", "openvpn"
}

/// Retrieves a list of configured VPN connections using nmcli.
#[tauri::command]
pub async fn get_vpn_connections() -> Result<Vec<VpnConnection>, String> {
    // nmcli -t -f UUID,NAME,TYPE,ACTIVE connection show
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "UUID,NAME,TYPE,ACTIVE", "connection", "show"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut vpns = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let uuid = parts[0].to_string();
            let name = parts[1].to_string();
            let conn_type = parts[2].to_string();
            let active_str = parts[3];

            // Filter for VPN and WireGuard connections
            if conn_type == "vpn" || conn_type == "wireguard" {
                // Determine active status: active_str is usually "--" or "no" if inactive
                let active = active_str != "no" && active_str != "--" && !active_str.is_empty();

                vpns.push(VpnConnection {
                    uuid,
                    name,
                    active,
                    type_name: conn_type,
                });
            }
        }
    }

    // Sort active connections to the top
    vpns.sort_by(|a, b| b.active.cmp(&a.active));

    Ok(vpns)
}

/// Activates a VPN connection by its UUID.
#[tauri::command]
pub async fn connect_vpn(uuid: String) -> Result<(), String> {
    let output = Command::new("nmcli")
        .args(&["connection", "up", &uuid])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Deactivates an active VPN connection by its UUID.
#[tauri::command]
pub async fn disconnect_vpn(uuid: String) -> Result<(), String> {
    let output = Command::new("nmcli")
        .args(&["connection", "down", &uuid])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Imports a VPN configuration file and optionally sets credentials.
#[tauri::command]
pub async fn import_vpn(
    file_path: String,
    vpn_type: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<String, String> {
    // Detect VPN type if not provided
    let type_str = if let Some(t) = vpn_type {
        t
    } else {
        if file_path.ends_with(".ovpn") {
            "openvpn".to_string()
        } else if file_path.ends_with(".conf") || file_path.ends_with(".wg") {
            "wireguard".to_string()
        } else {
            return Err("VPN type required and could not be detected from extension.".to_string());
        }
    };

    // Import the configuration using nmcli
    let output = Command::new("nmcli")
        .args(&[
            "connection",
            "import",
            "type",
            &type_str,
            "file",
            &file_path,
        ])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // If credentials are provided for OpenVPN, modify the connection properties
    if (username.is_some() || password.is_some()) && type_str == "openvpn" {
        // Extract UUID from nmcli output: "Connection '...' (UUID) successfully added."
        if let Some(start_idx) = stdout.rfind('(') {
            if let Some(end_idx) = stdout.rfind(')') {
                if start_idx < end_idx {
                    let uuid = &stdout[start_idx + 1..end_idx];

                    // Set Username
                    if let Some(user) = username {
                        let _ = Command::new("nmcli")
                            .args(&[
                                "connection",
                                "modify",
                                uuid,
                                "+vpn.data",
                                &format!("username={}", user),
                            ])
                            .output()
                            .await;
                    }

                    // Set Password and fix secret storage flags
                    if let Some(pass) = password {
                        let _ = Command::new("nmcli")
                            .args(&[
                                "connection",
                                "modify",
                                uuid,
                                "+vpn.secrets",
                                &format!("password={}", pass),
                            ])
                            .output()
                            .await;

                        let _ = Command::new("nmcli")
                            .args(&["connection", "modify", uuid, "vpn.secrets-flags", "0"])
                            .output()
                            .await;
                    }
                }
            }
        }
    }

    Ok("Imported successfully".to_string())
}
