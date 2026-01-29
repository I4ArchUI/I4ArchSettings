use bluer::{Adapter, AdapterEvent, Address, Session};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::sync::Mutex;

pub struct BluetoothState {
    // Bluetooth discovery is a RAII guard in bluer.
    // Dropping it stops discovery (if no one else is discovering).
    discovery_session: Mutex<Option<Box<dyn Stream<Item = AdapterEvent> + Send + Unpin>>>,
}

impl BluetoothState {
    pub fn new() -> Self {
        Self {
            discovery_session: Mutex::new(None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BluetoothDevice {
    mac: String,
    name: String,
    connected: bool,
    paired: bool,
    icon: Option<String>,
}

// Helper to get adapter
async fn get_adapter() -> Result<Adapter, String> {
    let session = Session::new().await.map_err(|e| e.to_string())?;
    let adapter = session.default_adapter().await.map_err(|e| e.to_string())?;
    Ok(adapter)
}

#[tauri::command]
pub async fn get_bluetooth_status() -> bool {
    match get_adapter().await {
        Ok(adapter) => adapter.is_powered().await.unwrap_or(false),
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn toggle_bluetooth(enable: bool) -> Result<(), String> {
    let adapter = get_adapter().await?;
    adapter
        .set_powered(enable)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn start_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let adapter = get_adapter().await?;

    // Ensure powered on
    if !adapter.is_powered().await.unwrap_or(false) {
        let _ = adapter.set_powered(true).await;
    }

    let mut guard = state.discovery_session.lock().await;
    if guard.is_none() {
        // Start discovery. This returns a stream, but effectively puts adapter in discovery mode
        // as long as the session object is alive.
        let discovery = adapter
            .discover_devices()
            .await
            .map_err(|e| e.to_string())?;
        *guard = Some(Box::new(discovery));
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let mut guard = state.discovery_session.lock().await;
    *guard = None; // Drop the guard to stop discovery
    Ok(())
}

#[tauri::command]
pub async fn get_bluetooth_devices() -> Result<Vec<BluetoothDevice>, String> {
    let adapter = get_adapter().await?;
    let device_addresses = adapter
        .device_addresses()
        .await
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for addr in device_addresses {
        if let Ok(device) = adapter.device(addr) {
            let name = device.name().await.unwrap_or(None);
            let alias = device.alias().await.unwrap_or_default();
            let addr_str = addr.to_string();

            // Prefer name, then alias, then address
            let display_name = name.or(Some(alias)).unwrap_or(addr_str.clone());

            // Simple Junk Filter
            if display_name
                .replace('-', ":")
                .eq_ignore_ascii_case(&addr_str)
            {
                continue;
            }

            let connected = device.is_connected().await.unwrap_or(false);
            let paired = device.is_paired().await.unwrap_or(false);

            let icon = device.icon().await.unwrap_or(None);

            result.push(BluetoothDevice {
                mac: addr_str,
                name: display_name,
                connected,
                paired,
                icon,
            });
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn connect_bluetooth(mac: String) -> Result<String, String> {
    let adapter = get_adapter().await?;
    let addr = Address::from_str(&mac).map_err(|_| "Invalid MAC address".to_string())?;
    let device = adapter.device(addr).map_err(|e| e.to_string())?;

    if !device.is_connected().await.unwrap_or(false) {
        device.connect().await.map_err(|e| e.to_string())?;
        Ok("Connected".to_string())
    } else {
        Ok("Already connected".to_string())
    }
}
