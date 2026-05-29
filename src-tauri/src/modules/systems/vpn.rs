use serde::{Deserialize, Serialize};

/// Represents a VPN connection with its metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VpnConnection {
    pub uuid: String,
    pub name: String,
    pub active: bool,
    pub type_name: String, // e.g. "wireguard", "openvpn", "openconnect", "l2tp", "pptp"
}

// =========================================================================
// LINUX D-BUS & NMRS IMPLEMENTATION
// =========================================================================
#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use zbus::zvariant::{Value, OwnedValue, ObjectPath};
    use tokio::process::Command;

    // Proxy for org.freedesktop.NetworkManager
    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager",
        default_service = "org.freedesktop.NetworkManager",
        default_path = "/org/freedesktop/NetworkManager"
    )]
    trait NetworkManager {
        fn activate_connection(
            &self,
            connection: zbus::zvariant::ObjectPath<'_>,
            device: zbus::zvariant::ObjectPath<'_>,
            specific_object: zbus::zvariant::ObjectPath<'_>,
        ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        fn deactivate_connection(
            &self,
            active_connection: zbus::zvariant::ObjectPath<'_>,
        ) -> zbus::Result<()>;

        #[zbus::interface(property)]
        fn active_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    }

    // Proxy for org.freedesktop.NetworkManager.Connection.Active
    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager.Connection.Active",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait ActiveConnection {
        #[zbus::interface(property)]
        fn uuid(&self) -> zbus::Result<String>;

        #[zbus::interface(property)]
        fn connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    }

    // Proxy for org.freedesktop.NetworkManager.Settings
    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager.Settings",
        default_service = "org.freedesktop.NetworkManager",
        default_path = "/org/freedesktop/NetworkManager/Settings"
    )]
    trait Settings {
        fn list_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
        fn add_connection(
            &self,
            connection: HashMap<String, HashMap<String, Value<'_>>>,
        ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    }

    // Proxy for org.freedesktop.NetworkManager.Settings.Connection
    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager.Settings.Connection",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait ConnectionSettings {
        fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;
    }

    /// Retrieves all configured VPN and WireGuard connections using zbus.
    pub async fn get_vpn_connections() -> Result<Vec<VpnConnection>, String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        
        let settings_proxy = SettingsProxy::new(&conn).await.map_err(|e| e.to_string())?;
        let paths = settings_proxy.list_connections().await.map_err(|e| e.to_string())?;
        
        // Fetch active connections to identify currently connected VPNs
        let nm_proxy = NetworkManagerProxy::new(&conn).await.map_err(|e| e.to_string())?;
        let active_paths = nm_proxy.active_connections().await.unwrap_or_default();
        
        let mut active_uuids = HashSet::new();
        for active_path in active_paths {
            if let Ok(active_conn_proxy) = ActiveConnectionProxy::builder(&conn).path(active_path).map_err(|e| e.to_string())?.build().await {
                if let Ok(uuid) = active_conn_proxy.uuid().await {
                    active_uuids.insert(uuid);
                }
            }
        }
        
        let mut vpns = Vec::new();
        
        for path in paths {
            if let Ok(conn_settings_proxy) = ConnectionSettingsProxy::builder(&conn).path(path.clone()).map_err(|e| e.to_string())?.build().await {
                if let Ok(settings) = conn_settings_proxy.get_settings().await {
                    if let Some(connection_section) = settings.get("connection") {
                        let conn_type = connection_section.get("type")
                            .and_then(|v| String::try_from(v.clone()).ok())
                            .unwrap_or_default();
                            
                        let uuid = connection_section.get("uuid")
                            .and_then(|v| String::try_from(v.clone()).ok())
                            .unwrap_or_default();
                            
                        let name = connection_section.get("id")
                            .and_then(|v| String::try_from(v.clone()).ok())
                            .unwrap_or_default();
                            
                        // Filter for VPN and WireGuard connections
                        if conn_type == "vpn" || conn_type == "wireguard" {
                            let mut friendly_type = conn_type.clone();
                            
                            if conn_type == "vpn" {
                                if let Some(vpn_section) = settings.get("vpn") {
                                    let service_type = vpn_section.get("service-type")
                                        .and_then(|v| String::try_from(v.clone()).ok())
                                        .unwrap_or_default();
                                        
                                    // Parse friendly service type
                                    friendly_type = if service_type.contains("openvpn") {
                                        "openvpn".to_string()
                                    } else if service_type.contains("openconnect") {
                                        "openconnect".to_string()
                                    } else if service_type.contains("l2tp") {
                                        "l2tp".to_string()
                                    } else if service_type.contains("pptp") {
                                        "pptp".to_string()
                                    } else {
                                        service_type.split('.').last().unwrap_or(&service_type).to_string()
                                    };
                                }
                            }
                            
                            let active = active_uuids.contains(&uuid);
                            
                            vpns.push(VpnConnection {
                                uuid,
                                name,
                                active,
                                type_name: friendly_type,
                            });
                        }
                    }
                }
            }
        }
        
        // Sort active connections to the top
        vpns.sort_by(|a, b| b.active.cmp(&a.active));
        Ok(vpns)
    }

    /// Activates a VPN connection by its UUID using zbus.
    pub async fn connect_vpn(uuid: String) -> Result<(), String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        
        let settings_proxy = SettingsProxy::new(&conn).await.map_err(|e| e.to_string())?;
        let paths = settings_proxy.list_connections().await.map_err(|e| e.to_string())?;
        
        let mut target_path = None;
        for path in paths {
            if let Ok(conn_settings_proxy) = ConnectionSettingsProxy::builder(&conn).path(path.clone()).map_err(|e| e.to_string())?.build().await {
                if let Ok(settings) = conn_settings_proxy.get_settings().await {
                    if let Some(connection_section) = settings.get("connection") {
                        let conn_uuid = connection_section.get("uuid")
                            .and_then(|v| String::try_from(v.clone()).ok())
                            .unwrap_or_default();
                            
                        if conn_uuid == uuid {
                            target_path = Some(path);
                            break;
                        }
                    }
                }
            }
        }
        
        let target_path = target_path.ok_or_else(|| "VPN connection not found".to_string())?;
        
        let nm_proxy = NetworkManagerProxy::new(&conn).await.map_err(|e| e.to_string())?;
        let empty_path = ObjectPath::try_from("/").unwrap();
        
        nm_proxy.activate_connection(target_path.into(), empty_path.clone(), empty_path)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    /// Deactivates a VPN connection by its UUID using zbus.
    pub async fn disconnect_vpn(uuid: String) -> Result<(), String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        
        let nm_proxy = NetworkManagerProxy::new(&conn).await.map_err(|e| e.to_string())?;
        let active_paths = nm_proxy.active_connections().await.map_err(|e| e.to_string())?;
        
        let mut target_active_path = None;
        for active_path in active_paths {
            if let Ok(active_conn_proxy) = ActiveConnectionProxy::builder(&conn).path(active_path.clone()).map_err(|e| e.to_string())?.build().await {
                if let Ok(conn_uuid) = active_conn_proxy.uuid().await {
                    if conn_uuid == uuid {
                        target_active_path = Some(active_path);
                        break;
                    }
                }
            }
        }
        
        let target_active_path = target_active_path.ok_or_else(|| "Active VPN connection not found".to_string())?;
        
        nm_proxy.deactivate_connection(target_active_path.into())
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    /// Creates a manual VPN connection by constructing settings via zbus.
    pub async fn create_manual_vpn(
        name: String,
        vpn_type: String,
        gateway: String,
        username: Option<String>,
        password: Option<String>,
        psk: Option<String>,
    ) -> Result<String, String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        let settings_proxy = SettingsProxy::new(&conn).await.map_err(|e| e.to_string())?;

        let mut settings = HashMap::new();

        // 1. Connection settings
        let mut connection = HashMap::new();
        connection.insert("id".to_string(), Value::new(&name));
        connection.insert("type".to_string(), Value::new("vpn"));
        settings.insert("connection".to_string(), connection);

        // 2. VPN settings
        let mut vpn = HashMap::new();
        
        let service_type = match vpn_type.as_str() {
            "openvpn" => "org.freedesktop.NetworkManager.openvpn",
            "openconnect" | "cisco" => "org.freedesktop.NetworkManager.openconnect",
            "l2tp" => "org.freedesktop.NetworkManager.l2tp",
            "pptp" => "org.freedesktop.NetworkManager.pptp",
            _ => return Err(format!("Unsupported VPN type for manual configuration: {}", vpn_type)),
        };
        vpn.insert("service-type".to_string(), Value::new(service_type));

        // Data map (a{ss})
        let mut data_map = HashMap::new();
        data_map.insert("gateway".to_string(), gateway.clone());
        
        if let Some(ref user) = username {
            if !user.is_empty() {
                data_map.insert("username".to_string(), user.clone());
            }
        }
        
        if vpn_type == "l2tp" {
            data_map.insert("ipsec-enabled".to_string(), "yes".to_string());
            if let Some(ref psk_val) = psk {
                if !psk_val.is_empty() {
                    data_map.insert("ipsec-psk".to_string(), psk_val.clone());
                }
            }
        }
        
        vpn.insert("data".to_string(), Value::new(data_map));

        // Secrets map (a{ss})
        if let Some(ref pass) = password {
            if !pass.is_empty() {
                let mut secrets_map = HashMap::new();
                secrets_map.insert("password".to_string(), pass.clone());
                vpn.insert("secrets".to_string(), Value::new(secrets_map));
            }
        }

        settings.insert("vpn".to_string(), vpn);

        // Save connection profile to NetworkManager Settings
        let _path = settings_proxy.add_connection(settings).await.map_err(|e| e.to_string())?;

        Ok("Created successfully".to_string())
    }

    /// Shell-based import of OpenVPN/Wireguard configurations (due to Lack of native D-Bus parser API).
    pub async fn import_vpn(
        file_path: String,
        vpn_type: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<String, String> {
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

        if (username.is_some() || password.is_some()) && type_str == "openvpn" {
            if let Some(start_idx) = stdout.rfind('(') {
                if let Some(end_idx) = stdout.rfind(')') {
                    if start_idx < end_idx {
                        let uuid = &stdout[start_idx + 1..end_idx];

                        if let Some(user) = username {
                            if !user.is_empty() {
                                let _ = Command::new("nmcli")
                                    .args(&["connection", "modify", uuid, "+vpn.data", &format!("username={}", user)])
                                    .output()
                                    .await;
                            }
                        }

                        if let Some(pass) = password {
                            if !pass.is_empty() {
                                let _ = Command::new("nmcli")
                                    .args(&["connection", "modify", uuid, "+vpn.secrets", &format!("password={}", pass)])
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
        }

        Ok("Imported successfully".to_string())
    }
}

// =========================================================================
// MOCK IMPLEMENTATION (WINDOWS / MACOS DEVELOPMENT FALLBACK)
// =========================================================================
#[cfg(not(target_os = "linux"))]
mod mock_impl {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;

    static MOCK_VPNS: Mutex<Option<Vec<VpnConnection>>> = Mutex::new(None);

    fn init_mocks() -> Vec<VpnConnection> {
        let mut list = MOCK_VPNS.lock().unwrap();
        if list.is_none() {
            *list = Some(vec![
                VpnConnection {
                    uuid: "mock-vpn-openvpn-1".to_string(),
                    name: "Office OpenVPN Tunnel".to_string(),
                    active: false,
                    type_name: "openvpn".to_string(),
                },
                VpnConnection {
                    uuid: "mock-vpn-wireguard-1".to_string(),
                    name: "Personal WG Cloud".to_string(),
                    active: true,
                    type_name: "wireguard".to_string(),
                },
                VpnConnection {
                    uuid: "mock-vpn-cisco-1".to_string(),
                    name: "Secure Corporate Cisco".to_string(),
                    active: false,
                    type_name: "openconnect".to_string(),
                },
            ]);
        }
        list.as_ref().unwrap().clone()
    }

    pub async fn get_vpn_connections() -> Result<Vec<VpnConnection>, String> {
        Ok(init_mocks())
    }

    pub async fn connect_vpn(uuid: String) -> Result<(), String> {
        let mut list = MOCK_VPNS.lock().unwrap();
        if let Some(ref mut vpns) = *list {
            for vpn in vpns.iter_mut() {
                if vpn.uuid == uuid {
                    vpn.active = true;
                } else if vpn.type_name != "wireguard" {
                    // Turn off other non-wireguard VPNs (typically only one active)
                    vpn.active = false;
                }
            }
        }
        Ok(())
    }

    pub async fn disconnect_vpn(uuid: String) -> Result<(), String> {
        let mut list = MOCK_VPNS.lock().unwrap();
        if let Some(ref mut vpns) = *list {
            for vpn in vpns.iter_mut() {
                if vpn.uuid == uuid {
                    vpn.active = false;
                }
            }
        }
        Ok(())
    }

    pub async fn create_manual_vpn(
        name: String,
        vpn_type: String,
        _gateway: String,
        _username: Option<String>,
        _password: Option<String>,
        _psk: Option<String>,
    ) -> Result<String, String> {
        init_mocks();
        let mut list = MOCK_VPNS.lock().unwrap();
        if let Some(ref mut vpns) = *list {
            let friendly_type = match vpn_type.as_str() {
                "openvpn" => "openvpn",
                "openconnect" | "cisco" => "openconnect",
                "l2tp" => "l2tp",
                "pptp" => "pptp",
                _ => &vpn_type,
            };
            vpns.push(VpnConnection {
                uuid: format!("mock-vpn-{}-{}", friendly_type, rand::random::<u32>()),
                name,
                active: false,
                type_name: friendly_type.to_string(),
            });
        }
        Ok("Created successfully".to_string())
    }

    pub async fn import_vpn(
        file_path: String,
        vpn_type: Option<String>,
        _username: Option<String>,
        _password: Option<String>,
    ) -> Result<String, String> {
        init_mocks();
        let mut list = MOCK_VPNS.lock().unwrap();
        if let Some(ref mut vpns) = *list {
            let detected_type = vpn_type.unwrap_or_else(|| {
                if file_path.ends_with(".ovpn") {
                    "openvpn".to_string()
                } else {
                    "wireguard".to_string()
                }
            });
            let filename = file_path.split(/[/\\]/).last().unwrap_or("imported_profile");
            vpns.push(VpnConnection {
                uuid: format!("mock-vpn-{}-{}", detected_type, rand::random::<u32>()),
                name: filename.replace(".ovpn", "").replace(".conf", "").replace(".wg", ""),
                active: false,
                type_name: detected_type,
            });
        }
        Ok("Imported successfully".to_string())
    }
}

// =========================================================================
// EXPOSED TAURI COMMAND ENTRY POINTS
// =========================================================================

#[tauri::command]
pub async fn get_vpn_connections() -> Result<Vec<VpnConnection>, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::get_vpn_connections().await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::get_vpn_connections().await }
}

#[tauri::command]
pub async fn connect_vpn(uuid: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    { linux_impl::connect_vpn(uuid).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::connect_vpn(uuid).await }
}

#[tauri::command]
pub async fn disconnect_vpn(uuid: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    { linux_impl::disconnect_vpn(uuid).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::disconnect_vpn(uuid).await }
}

#[tauri::command]
pub async fn import_vpn(
    file_path: String,
    vpn_type: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::import_vpn(file_path, vpn_type, username, password).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::import_vpn(file_path, vpn_type, username, password).await }
}

#[tauri::command]
pub async fn create_manual_vpn(
    name: String,
    vpn_type: String,
    gateway: String,
    username: Option<String>,
    password: Option<String>,
    psk: Option<String>,
) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::create_manual_vpn(name, vpn_type, gateway, username, password, psk).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::create_manual_vpn(name, vpn_type, gateway, username, password, psk).await }
}
