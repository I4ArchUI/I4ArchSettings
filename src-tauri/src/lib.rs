mod modules;

use modules::apps::apps::{
    get_installed_apps, get_installed_packages, uninstall_app, uninstall_package,
};
use modules::apps::updates::{check_updates, update_system};
use modules::hyprland::appearance::{
    apply_appearance_conf, get_current_appearance_config, get_cursor_themes, get_gtk_themes_list,
    get_hyprland_config, save_hyprland_config,
};
use modules::hyprland::display::{get_displays, save_displays};
use modules::hyprland::env::{get_env_vars, save_env_vars};
use modules::hyprland::keybinds::{get_keybinds, save_keybinds};
use modules::hyprland::kitty::set_kitty_theme;
use modules::hyprland::startup::{get_startup_commands, save_startup_commands};
use modules::hyprland::wallpaper::{
    get_current_wallpaper_path, get_wallpaper_base64, set_wallpaper,
};
use modules::hyprland::waybar::{get_waybar_position, set_waybar_position};
use modules::settings::{get_app_settings, save_app_settings};
use modules::systems::bluetooth::{
    connect_bluetooth, get_bluetooth_devices, get_bluetooth_status, start_scan, stop_scan,
    toggle_bluetooth, BluetoothState,
};
use modules::systems::system::{check_app_installed, get_gtk_theme, get_system_info};
use modules::systems::vpn::{connect_vpn, disconnect_vpn, get_vpn_connections, import_vpn};
use modules::systems::wifi::{
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
            check_app_installed,
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
            set_kitty_theme,
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
