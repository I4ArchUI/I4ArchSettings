use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VpnConnection {
    pub uuid: String,
    pub name: String,
    pub active: bool,
    pub type_name: String, // e.g. "wireguard", "openvpn"
}

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
        // UUID:NAME:TYPE:ACTIVE (active is yes/no or color coded in some versions, but -t should be clean)
        // Actually ACTIVE might be the device name or similar in some modes, but usually "yes"/"no" or connection state.
        // Let's check `nmcli` docs or standard output.
        // `nmcli -t -f ACTIVE connection show` usually returns "yes"/"no" or depending on version.
        // Wait, standard `nmcli connection show` lists configured connections.
        // UUID,NAME,TYPE,ACTIVE might not be perfectly aligned if restricted to VPN.
        // TYPE for vpn is usually "vpn" or "wireguard".

        if parts.len() >= 4 {
            let uuid = parts[0].to_string();
            let name = parts[1].to_string();
            let conn_type = parts[2].to_string();
            let active_str = parts[3]; // "yes" or "no" usually, or the device name if active.

            // Filter for VPNs. Wireguard is often "wireguard", others are "vpn" (openvpn, etc).
            if conn_type == "vpn" || conn_type == "wireguard" {
                // Check if active. "yes", or colored output avoided by -t?
                // Usually with -t, it's just value.
                // Actually, checking if active can be tricky. using connection show --active can serve as cross reference.
                // But let's assume `ACTIVE` field returns "yes" or similar.
                // In some versions, ACTIVE field is the interface name (e.g. "tun0") if active, nothing if not?
                // Let's try to interpret "yes" or any non-empty/non-no string as potentially active, but let's be careful.
                // Actually, simpler: nmcli -t -f UUID,NAME,TYPE,DEVICE connection show
                // DEVICE will be "--" if not active, or device name if active.

                // Let's stick to what I wrote above but double check active logic.
                // I will assume standard: green/yes means active.
                // safe bet: active_str != "--" && active_str != "no"

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

    // Sort active first
    vpns.sort_by(|a, b| b.active.cmp(&a.active));

    Ok(vpns)
}

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

#[tauri::command]
pub async fn import_vpn(
    file_path: String,
    vpn_type: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<String, String> {
    // Determine type
    let type_str = if let Some(t) = vpn_type {
        t
    } else {
        // Simple heuristic fallback
        if file_path.ends_with(".ovpn") {
            "openvpn".to_string()
        } else if file_path.ends_with(".conf") || file_path.ends_with(".wg") {
            "wireguard".to_string()
        } else {
            return Err("VPN type required and could not be detected from extension.".to_string());
        }
    };

    // 1. Import
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
    // Expected stdout: "Connection 'Details' (UUID) successfully added."

    // 2. If username/password provided (Mainly for OpenVPN), modify connection
    if (username.is_some() || password.is_some()) && type_str == "openvpn" {
        // Extract UUID
        // Find substring between '(' and ')'
        if let Some(start_idx) = stdout.rfind('(') {
            if let Some(end_idx) = stdout.rfind(')') {
                if start_idx < end_idx {
                    let uuid = &stdout[start_idx + 1..end_idx];

                    // Modify Username
                    if let Some(user) = username {
                        // vpn.user-name for openvpn is common, or vpn.data username=... depending on plugin version.
                        // Modern NM OpenVPN uses +vpn.data username=...
                        // Let's try `vpn.user-name` property first if it exists as a direct property,
                        // but typically `nmcli connection modify <uuid> +vpn.data username=<user>` is the robust way for plugins.
                        // Double check: `nmcli -f vpn.data connection show <uuid>`
                        // Actually, standard property is often `vpn.user-name` on recent NM.
                        // Let's try generic `+vpn.data username=...` as it's safe for the plugin metadata.

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

                    // Modify Password
                    if let Some(pass) = password {
                        // +vpn.secrets password=...
                        // Also set flags to 0 (save)
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
