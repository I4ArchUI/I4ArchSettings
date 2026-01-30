use bluer::{Adapter, AdapterEvent, Address, Session};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::sync::Mutex;

/// Manages the state for Bluetooth operations.
pub struct BluetoothState {
    /// Bluetooth discovery session guard.
    /// Dropping this automatically stops discovery.
    discovery_session: Mutex<Option<Box<dyn Stream<Item = AdapterEvent> + Send + Unpin>>>,
}

impl BluetoothState {
    pub fn new() -> Self {
        Self {
            discovery_session: Mutex::new(None),
        }
    }
}

/// Represents a Bluetooth device with its metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct BluetoothDevice {
    mac: String,
    name: String,
    connected: bool,
    paired: bool,
    icon: Option<String>,
}

/// Helper function to retrieve the default Bluetooth adapter.
async fn get_adapter() -> Result<Adapter, String> {
    let session = Session::new().await.map_err(|e| e.to_string())?;
    let adapter = session.default_adapter().await.map_err(|e| e.to_string())?;
    Ok(adapter)
}

/// Checks if Bluetooth is currently powered on.
#[tauri::command]
pub async fn get_bluetooth_status() -> bool {
    match get_adapter().await {
        Ok(adapter) => adapter.is_powered().await.unwrap_or(false),
        Err(_) => false,
    }
}

/// Toggles Bluetooth power state.
#[tauri::command]
pub async fn toggle_bluetooth(enable: bool) -> Result<(), String> {
    let adapter = get_adapter().await?;
    adapter
        .set_powered(enable)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Starts scanning for nearby Bluetooth devices.
#[tauri::command]
pub async fn start_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let adapter = get_adapter().await?;

    // Ensure adapter is powered on before scanning
    if !adapter.is_powered().await.unwrap_or(false) {
        let _ = adapter.set_powered(true).await;
    }

    let mut guard = state.discovery_session.lock().await;
    if guard.is_none() {
        // Initialize discovery session and store it in state
        let discovery = adapter
            .discover_devices()
            .await
            .map_err(|e| e.to_string())?;
        *guard = Some(Box::new(discovery));
    }

    Ok(())
}

/// Stops the current Bluetooth device scan.
#[tauri::command]
pub async fn stop_scan(state: tauri::State<'_, BluetoothState>) -> Result<(), String> {
    let mut guard = state.discovery_session.lock().await;
    *guard = None; // Drop the guard to terminate discovery
    Ok(())
}

/// Retrieves a list of discovered Bluetooth devices.
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

            // Resolve display name: Name > Alias > MAC Address
            let display_name = name.or(Some(alias)).unwrap_or(addr_str.clone());

            // Filter out internal/junk entries that match the MAC address format
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

/// Attempts to connect to a Bluetooth device by its MAC address.
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
