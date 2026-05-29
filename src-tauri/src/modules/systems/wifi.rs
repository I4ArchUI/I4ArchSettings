use serde::{Deserialize, Serialize};

/// Represents a Wi-Fi network with its signal strength and status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub security: String,
    pub bars: String,
    pub signal: u8,
    pub active: bool,
}

/// Configuration for a Wi-Fi connection (IPv4 settings).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiConfig {
    pub method: String,
    pub ip_address: String,
    pub prefix: i32,
    pub gateway: String,
    pub dns: String,
    pub bssid: Option<String>,
    pub frequency: Option<String>,
    pub speed: Option<String>,
    pub interface: Option<String>,
    pub mac_address: Option<String>,
}

// =========================================================================
// LINUX D-BUS & NMRS IMPLEMENTATION
// =========================================================================
#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;
    use nmrs::NetworkManager;
    use zbus::zvariant::{Value, OwnedValue};
    use std::collections::HashMap;

    // Define ZBus D-Bus proxies for NetworkManager Settings interface to handle IP configurations
    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager.Settings",
        default_service = "org.freedesktop.NetworkManager",
        default_path = "/org/freedesktop/NetworkManager/Settings"
    )]
    trait Settings {
        fn list_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    }

    #[zbus::proxy(
        interface = "org.freedesktop.NetworkManager.Settings.Connection",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait ConnectionSettings {
        fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;
        fn update(&self, properties: HashMap<String, HashMap<String, Value<'_>>>) -> zbus::Result<()>;
    }

    // Helper to search for a connection profile by its ID (SSID)
    async fn find_connection_path(conn: &zbus::Connection, ssid: &str) -> Option<zbus::zvariant::OwnedObjectPath> {
        let settings_proxy = SettingsProxy::new(conn).await.ok()?;
        let paths = settings_proxy.list_connections().await.ok()?;
        for path in paths {
            if let Ok(conn_settings_proxy) = ConnectionSettingsProxy::builder(conn).path(path.clone()).ok()?.build().await {
                if let Ok(settings) = conn_settings_proxy.get_settings().await {
                    if let Some(connection_section) = settings.get("connection") {
                        if let Some(id_val) = connection_section.get("id") {
                            if let Ok(id_str) = String::try_from(id_val.clone()) {
                                if id_str == ssid {
                                    return Some(path);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub async fn get_wifi_status() -> bool {
        let Ok(conn) = zbus::Connection::system().await else { return false; };
        let Ok(proxy) = zbus::Proxy::new(
            &conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        ).await else { return false; };
        proxy.get_property::<bool>("WirelessEnabled").await.unwrap_or(false)
    }

    pub async fn toggle_wifi(enable: bool) -> Result<(), String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        let proxy = zbus::Proxy::new(
            &conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        ).await.map_err(|e| e.to_string())?;
        proxy.set_property("WirelessEnabled", enable).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn scan_wifi() -> Result<Vec<WifiNetwork>, String> {
        let nm = NetworkManager::new().await.map_err(|e| e.to_string())?;
        let networks = nm.list_networks(None).await.map_err(|e| e.to_string())?;
        let current_ssid = nm.current_ssid().await;

        let mut result = Vec::new();
        for net in networks {
            if net.ssid.is_empty() {
                continue;
            }

            let active = current_ssid.as_ref().map(|s| s == &net.ssid).unwrap_or(false);
            
            // Map security using format debug to be highly compile-safe
            let security_str = format!("{:?}", net.security);
            let security = if security_str.contains("None") {
                "".to_string()
            } else {
                "WPA/WPA2".to_string()
            };

            let signal = net.strength.unwrap_or(0);
            let bars = if signal > 75 {
                "icon-wifi-strong"
            } else if signal > 40 {
                "icon-wifi-medium"
            } else {
                "icon-wifi-weak"
            };

            result.push(WifiNetwork {
                ssid: net.ssid.clone(),
                security,
                bars: bars.to_string(),
                signal,
                active,
            });
        }

        // Deduplicate SSID
        let mut map = HashMap::new();
        for item in result {
            match map.get(&item.ssid) {
                Some(existing) => {
                    let is_better = if item.active && !existing.active {
                        true
                    } else if item.active == existing.active {
                        item.signal > existing.signal
                    } else {
                        false
                    };
                    if is_better {
                        map.insert(item.ssid.clone(), item);
                    }
                }
                None => {
                    map.insert(item.ssid.clone(), item);
                }
            }
        }

        let mut final_list: Vec<WifiNetwork> = map.into_values().collect();
        final_list.sort_by(|a, b| b.active.cmp(&a.active).then(b.signal.cmp(&a.signal)));
        Ok(final_list)
    }

    pub async fn connect_wifi(ssid: String, password: Option<String>, username: Option<String>) -> Result<String, String> {
        let nm = NetworkManager::new().await.map_err(|e| e.to_string())?;
        let security = match password {
            Some(ref pwd) if !pwd.is_empty() => {
                if let Some(ref user) = username {
                    if !user.is_empty() {
                        nmrs::WifiSecurity::WpaEap {
                            identity: user.clone(),
                            password: pwd.clone(),
                        }
                    } else {
                        nmrs::WifiSecurity::WpaPsk { psk: pwd.clone() }
                    }
                } else {
                    nmrs::WifiSecurity::WpaPsk { psk: pwd.clone() }
                }
            }
            _ => nmrs::WifiSecurity::None,
        };

        nm.connect(&ssid, None, security).await.map_err(|e| e.to_string())?;
        Ok("Connected successfully".to_string())
    }

    pub async fn get_wifi_config(ssid: String) -> Result<WifiConfig, String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        
        let mut method = "auto".to_string();
        let mut ip_address = "".to_string();
        let mut prefix = 24;
        let mut gateway = "".to_string();
        let mut dns = "".to_string();

        if let Some(path) = find_connection_path(&conn, &ssid).await {
            if let Ok(proxy) = ConnectionSettingsProxy::builder(&conn).path(path).map_err(|e| e.to_string())?.build().await {
                if let Ok(settings) = proxy.get_settings().await {
                    if let Some(ipv4_section) = settings.get("ipv4") {
                        if let Some(method_val) = ipv4_section.get("method") {
                            method = String::try_from(method_val.clone()).unwrap_or_else(|_| "auto".to_string());
                        }
                        
                        if let Some(gateway_val) = ipv4_section.get("gateway") {
                            gateway = String::try_from(gateway_val.clone()).unwrap_or_default();
                        }

                        // Read DNS
                        if let Some(dns_val) = ipv4_section.get("dns") {
                            if let Ok(dns_ips) = Vec::<u32>::try_from(dns_val.clone()) {
                                let dns_strs: Vec<String> = dns_ips.iter().map(|&ip| {
                                    let bytes = ip.to_be_bytes();
                                    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
                                }).collect();
                                dns = dns_strs.join(", ");
                            }
                        }

                        // Read static addresses
                        if let Some(addresses_val) = ipv4_section.get("addresses") {
                            if let Ok(addrs) = Vec::<Vec<u32>>::try_from(addresses_val.clone()) {
                                if let Some(addr_vec) = addrs.first() {
                                    if addr_vec.len() >= 2 {
                                        let ip_u32 = addr_vec[0];
                                        let pref = addr_vec[1];
                                        let bytes = ip_u32.to_ne_bytes();
                                        ip_address = format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]);
                                        prefix = pref as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Live connection statistics via D-Bus properties
        let mut bssid = None;
        let mut frequency = None;
        let mut speed = None;
        let mut interface = None;
        let mut mac_address = None;

        if let Ok(nm_proxy) = zbus::Proxy::new(
            &conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        ).await {
            if let Ok(device_paths) = nm_proxy.get_property::<Vec<zbus::zvariant::OwnedObjectPath>>("Devices").await {
                for dev_path in device_paths {
                    if let Ok(dev_proxy) = zbus::Proxy::new(
                        &conn,
                        "org.freedesktop.NetworkManager",
                        &dev_path,
                        "org.freedesktop.NetworkManager.Device",
                    ).await {
                        let dev_type = dev_proxy.get_property::<u32>("DeviceType").await.unwrap_or(0);
                        let state = dev_proxy.get_property::<u32>("State").await.unwrap_or(0);

                        if dev_type == 2 && state == 100 { // 2 = Wireless, 100 = Activated
                            let iface = dev_proxy.get_property::<String>("Interface").await.unwrap_or_default();
                            let hw_addr = dev_proxy.get_property::<String>("HwAddress").await.unwrap_or_default();

                            if let Ok(wifi_proxy) = zbus::Proxy::new(
                                &conn,
                                "org.freedesktop.NetworkManager",
                                &dev_path,
                                "org.freedesktop.NetworkManager.Device.Wireless",
                            ).await {
                                if let Ok(ap_path) = wifi_proxy.get_property::<zbus::zvariant::OwnedObjectPath>("ActiveAccessPoint").await {
                                    if ap_path.as_str() != "/" {
                                        if let Ok(ap_proxy) = zbus::Proxy::new(
                                            &conn,
                                            "org.freedesktop.NetworkManager",
                                            &ap_path,
                                            "org.freedesktop.NetworkManager.AccessPoint",
                                        ).await {
                                            let ssid_bytes = ap_proxy.get_property::<Vec<u8>>("Ssid").await.unwrap_or_default();
                                            let ap_ssid = String::from_utf8(ssid_bytes).unwrap_or_default();

                                            // If the active AP's SSID matches, populate details
                                            if ap_ssid == ssid {
                                                interface = Some(iface);
                                                mac_address = Some(hw_addr);
                                                bssid = Some(ap_proxy.get_property::<String>("HwAddress").await.unwrap_or_default());
                                                
                                                let freq_mhz = ap_proxy.get_property::<u32>("Frequency").await.unwrap_or(0);
                                                if freq_mhz > 0 {
                                                    let band = if freq_mhz > 4900 { "5 GHz" } else { "2.4 GHz" };
                                                    frequency = Some(format!("{} ({} MHz)", band, freq_mhz));
                                                }

                                                let bitrate = wifi_proxy.get_property::<u32>("Bitrate").await.unwrap_or(0);
                                                if bitrate > 0 {
                                                    speed = Some(format!("{} Mbps", bitrate / 1000));
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(WifiConfig {
            method,
            ip_address,
            prefix,
            gateway,
            dns,
            bssid,
            frequency,
            speed,
            interface,
            mac_address,
        })
    }

    pub async fn set_wifi_config(ssid: String, config: WifiConfig) -> Result<(), String> {
        let conn = zbus::Connection::system().await.map_err(|e| e.to_string())?;
        
        let Some(path) = find_connection_path(&conn, &ssid).await else {
            return Err("Connection profile not found".to_string());
        };

        let proxy = ConnectionSettingsProxy::builder(&conn)
            .path(path)
            .map_err(|e| e.to_string())?
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let mut settings = proxy.get_settings().await.map_err(|e| e.to_string())?;

        let mut ipv4 = HashMap::new();
        ipv4.insert("method".to_string(), Value::from(config.method.clone()));

        if config.method == "manual" {
            let ip_parts: Vec<&str> = config.ip_address.split('.').collect();
            if ip_parts.len() == 4 {
                let b0 = ip_parts[0].parse::<u8>().unwrap_or(0);
                let b1 = ip_parts[1].parse::<u8>().unwrap_or(0);
                let b2 = ip_parts[2].parse::<u8>().unwrap_or(0);
                let b3 = ip_parts[3].parse::<u8>().unwrap_or(0);
                let ip_u32 = u32::from_ne_bytes([b0, b1, b2, b3]);

                let gw_parts: Vec<&str> = config.gateway.split('.').collect();
                let gw_u32 = if gw_parts.len() == 4 {
                    let gb0 = gw_parts[0].parse::<u8>().unwrap_or(0);
                    let gb1 = gw_parts[1].parse::<u8>().unwrap_or(0);
                    let gb2 = gw_parts[2].parse::<u8>().unwrap_or(0);
                    let gb3 = gw_parts[3].parse::<u8>().unwrap_or(0);
                    u32::from_ne_bytes([gb0, gb1, gb2, gb3])
                } else {
                    0
                };

                let addr_data = vec![vec![ip_u32, config.prefix as u32, gw_u32]];
                ipv4.insert("addresses".to_string(), Value::from(addr_data));
            }
            if !config.gateway.is_empty() {
                ipv4.insert("gateway".to_string(), Value::from(config.gateway.clone()));
            }
        } else {
            ipv4.insert("addresses".to_string(), Value::from(Vec::<Vec<u32>>::new()));
            ipv4.insert("gateway".to_string(), Value::from(""));
        }

        if !config.dns.is_empty() {
            let mut dns_ips = Vec::new();
            for ip_str in config.dns.split(',') {
                let ip_trimmed = ip_str.trim();
                let parts: Vec<&str> = ip_trimmed.split('.').collect();
                if parts.len() == 4 {
                    let b0 = parts[0].parse::<u8>().unwrap_or(0);
                    let b1 = parts[1].parse::<u8>().unwrap_or(0);
                    let b2 = parts[2].parse::<u8>().unwrap_or(0);
                    let b3 = parts[3].parse::<u8>().unwrap_or(0);
                    dns_ips.push(u32::from_ne_bytes([b0, b1, b2, b3]));
                }
            }
            ipv4.insert("dns".to_string(), Value::from(dns_ips));
        } else {
            ipv4.insert("dns".to_string(), Value::from(Vec::<u32>::new()));
        }

        // Construct update dictionary map
        let mut update_map = HashMap::new();
        for (sec_name, sec_data) in settings {
            let mut sec_map = HashMap::new();
            if sec_name == "ipv4" {
                for (k, v) in sec_data {
                    sec_map.insert(k.clone(), Value::from(v));
                }
                for (k, v) in ipv4.clone() {
                    sec_map.insert(k, v);
                }
            } else {
                for (k, v) in sec_data {
                    sec_map.insert(k.clone(), Value::from(v));
                }
            }
            update_map.insert(sec_name, sec_map);
        }

        proxy.update(update_map).map_err(|e| e.to_string())?;
        Ok(())
    }
}

// =========================================================================
// WINDOWS/MACOS SIMULATED/MOCK IMPLEMENTATION FOR DEVELOPMENT
// =========================================================================
#[cfg(not(target_os = "linux"))]
mod mock_impl {
    use super::*;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    fn wifi_enabled() -> &'static Mutex<bool> {
        static WIFI_ENABLED: OnceLock<Mutex<bool>> = OnceLock::new();
        WIFI_ENABLED.get_or_init(|| Mutex::new(true))
    }

    fn connected_ssid() -> &'static Mutex<String> {
        static CONNECTED_SSID: OnceLock<Mutex<String>> = OnceLock::new();
        CONNECTED_SSID.get_or_init(|| Mutex::new("Mock_Wifi_5G".to_string()))
    }

    fn configs() -> &'static Mutex<std::collections::HashMap<String, WifiConfig>> {
        static CONFIGS: OnceLock<Mutex<std::collections::HashMap<String, WifiConfig>>> = OnceLock::new();
        CONFIGS.get_or_init(|| Mutex::new({
            let mut map = std::collections::HashMap::new();
            map.insert("Mock_Wifi_5G".to_string(), WifiConfig {
                method: "auto".to_string(),
                ip_address: "192.168.1.123".to_string(),
                prefix: 24,
                gateway: "192.168.1.1".to_string(),
                dns: "8.8.8.8, 1.1.1.1".to_string(),
                bssid: Some("00:11:22:33:44:55".to_string()),
                frequency: Some("5 GHz (5180 MHz)".to_string()),
                speed: Some("866 Mbps".to_string()),
                interface: Some("wlan0".to_string()),
                mac_address: Some("a0:b1:c2:d3:e4:f5".to_string()),
            });
            map
        }))
    }

    pub async fn get_wifi_status() -> bool {
        *wifi_enabled().lock().unwrap()
    }

    pub async fn toggle_wifi(enable: bool) -> Result<(), String> {
        let mut lock = wifi_enabled().lock().unwrap();
        *lock = enable;
        Ok(())
    }

    pub async fn scan_wifi() -> Result<Vec<WifiNetwork>, String> {
        if !*wifi_enabled().lock().unwrap() {
            return Ok(Vec::new());
        }

        let active_ssid = connected_ssid().lock().unwrap().clone();
        Ok(vec![
            WifiNetwork {
                ssid: "Mock_Wifi_5G".to_string(),
                security: "WPA2".to_string(),
                bars: "icon-wifi-strong".to_string(),
                signal: 92,
                active: active_ssid == "Mock_Wifi_5G",
            },
            WifiNetwork {
                ssid: "Demo_Enterprise_802.1X".to_string(),
                security: "WPA-Enterprise".to_string(),
                bars: "icon-wifi-strong".to_string(),
                signal: 88,
                active: active_ssid == "Demo_Enterprise_802.1X",
            },
            WifiNetwork {
                ssid: "Arch_AP_Guest".to_string(),
                security: "WPA/WPA2".to_string(),
                bars: "icon-wifi-medium".to_string(),
                signal: 68,
                active: active_ssid == "Arch_AP_Guest",
            },
            WifiNetwork {
                ssid: "Coffee_Shop_Free".to_string(),
                security: "".to_string(),
                bars: "icon-wifi-weak".to_string(),
                signal: 35,
                active: active_ssid == "Coffee_Shop_Free",
            },
        ])
    }

    pub async fn connect_wifi(ssid: String, password: Option<String>, username: Option<String>) -> Result<String, String> {
        if ssid == "Mock_Wifi_5G" || ssid == "Arch_AP_Guest" || ssid == "Demo_Enterprise_802.1X" {
            if ssid == "Demo_Enterprise_802.1X" {
                if let Some(ref user) = username {
                    if user.is_empty() {
                        return Err("Username/Identity is required for Enterprise".to_string());
                    }
                } else {
                    return Err("Username/Identity is required for Enterprise".to_string());
                }
            }
            if let Some(pwd) = password {
                if pwd.is_empty() {
                    return Err("Password cannot be empty".to_string());
                }
                if pwd == "error" {
                    return Err("Authentication failed: incorrect password".to_string());
                }
            } else {
                return Err("Security credentials required".to_string());
            }
        }

        let mut active = connected_ssid().lock().unwrap();
        *active = ssid.clone();
        Ok("Connected successfully".to_string())
    }

    pub async fn get_wifi_config(ssid: String) -> Result<WifiConfig, String> {
        let lock = configs().lock().unwrap();
        if let Some(conf) = lock.get(&ssid) {
            Ok(conf.clone())
        } else {
            let (bssid, frequency, speed) = match ssid.as_str() {
                "Demo_Enterprise_802.1X" => (
                    Some("00:11:22:aa:bb:cc".to_string()),
                    Some("5 GHz (5240 MHz)".to_string()),
                    Some("1300 Mbps".to_string()),
                ),
                "Arch_AP_Guest" => (
                    Some("00:11:22:66:77:88".to_string()),
                    Some("2.4 GHz (2437 MHz)".to_string()),
                    Some("144 Mbps".to_string()),
                ),
                "Coffee_Shop_Free" => (
                    Some("00:11:22:11:22:33".to_string()),
                    Some("2.4 GHz (2412 MHz)".to_string()),
                    Some("54 Mbps".to_string()),
                ),
                _ => (
                    Some("00:11:22:33:44:55".to_string()),
                    Some("5 GHz (5180 MHz)".to_string()),
                    Some("866 Mbps".to_string()),
                ),
            };
            Ok(WifiConfig {
                method: "auto".to_string(),
                ip_address: "".to_string(),
                prefix: 24,
                gateway: "".to_string(),
                dns: "".to_string(),
                bssid,
                frequency,
                speed,
                interface: Some("wlan0".to_string()),
                mac_address: Some("a0:b1:c2:d3:e4:f5".to_string()),
            })
        }
    }

    pub async fn set_wifi_config(ssid: String, config: WifiConfig) -> Result<(), String> {
        let mut lock = configs().lock().unwrap();
        lock.insert(ssid, config);
        Ok(())
    }
}

// =========================================================================
// EXPOSED TAURI COMMAND ENTRY POINTS
// =========================================================================

#[tauri::command]
pub async fn get_wifi_status() -> bool {
    #[cfg(target_os = "linux")]
    { linux_impl::get_wifi_status().await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::get_wifi_status().await }
}

#[tauri::command]
pub async fn toggle_wifi(enable: bool) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    { linux_impl::toggle_wifi(enable).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::toggle_wifi(enable).await }
}

#[tauri::command]
pub async fn scan_wifi() -> Result<Vec<WifiNetwork>, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::scan_wifi().await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::scan_wifi().await }
}

#[tauri::command]
pub async fn connect_wifi(ssid: String, password: Option<String>, username: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::connect_wifi(ssid, password, username).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::connect_wifi(ssid, password, username).await }
}

#[tauri::command]
pub async fn get_wifi_config(ssid: String) -> Result<WifiConfig, String> {
    #[cfg(target_os = "linux")]
    { linux_impl::get_wifi_config(ssid).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::get_wifi_config(ssid).await }
}

#[tauri::command]
pub async fn set_wifi_config(ssid: String, config: WifiConfig) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    { linux_impl::set_wifi_config(ssid, config).await }
    #[cfg(not(target_os = "linux"))]
    { mock_impl::set_wifi_config(ssid, config).await }
}
