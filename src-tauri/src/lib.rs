mod modules;

use modules::appearance::{
    apply_appearance_conf, get_current_appearance_config, get_cursor_themes, get_gtk_themes_list,
    get_hyprland_config, save_hyprland_config,
};
use modules::apps::{get_installed_apps, get_installed_packages, uninstall_app, uninstall_package};
use modules::bluetooth::{
    connect_bluetooth, get_bluetooth_devices, get_bluetooth_status, start_scan, stop_scan,
    toggle_bluetooth, BluetoothState,
};
use modules::display::{get_displays, save_displays};
use modules::env::{get_env_vars, save_env_vars};
use modules::keybinds::{get_keybinds, save_keybinds};
use modules::settings::{get_app_settings, save_app_settings};
use modules::startup::{get_startup_commands, save_startup_commands};
use modules::system::{get_gtk_theme, get_system_info};
use modules::updates::{check_updates, update_system};
use modules::vpn::{connect_vpn, disconnect_vpn, get_vpn_connections, import_vpn};
use modules::wallpaper::{get_current_wallpaper_path, get_wallpaper_base64, set_wallpaper};
use modules::waybar::{get_waybar_position, set_waybar_position};
use modules::wifi::{
    connect_wifi, get_wifi_config, get_wifi_status, scan_wifi, set_wifi_config, toggle_wifi,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(BluetoothState::new())
        .invoke_handler(tauri::generate_handler![
            get_wifi_status,
            toggle_wifi,
            scan_wifi,
            connect_wifi,
            get_wifi_config,
            set_wifi_config,
            get_bluetooth_status,
            toggle_bluetooth,
            start_scan,
            stop_scan,
            get_bluetooth_devices,
            connect_bluetooth,
            get_gtk_theme,
            get_system_info,
            set_wallpaper,
            get_current_wallpaper_path,
            get_wallpaper_base64,
            get_app_settings,
            save_app_settings,
            get_waybar_position,
            set_waybar_position,
            get_displays,
            save_displays,
            get_cursor_themes,
            get_gtk_themes_list,
            apply_appearance_conf,
            get_current_appearance_config,
            get_hyprland_config,
            save_hyprland_config,
            get_startup_commands,
            save_startup_commands,
            get_keybinds,
            save_keybinds,
            get_env_vars,
            save_env_vars,
            get_vpn_connections,
            connect_vpn,
            disconnect_vpn,
            import_vpn,
            check_updates,
            update_system,
            get_installed_apps,
            get_installed_packages,
            uninstall_package,
            uninstall_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
