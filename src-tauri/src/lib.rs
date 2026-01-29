mod modules;

use modules::appearance::{
    apply_appearance_conf, get_current_appearance_config, get_cursor_themes, get_gtk_themes_list,
};
use modules::bluetooth::{
    connect_bluetooth, get_bluetooth_status, scan_bluetooth, toggle_bluetooth,
};
use modules::display::{get_displays, save_displays};
use modules::settings::{get_app_settings, save_app_settings};
use modules::system::{get_gtk_theme, get_system_info};
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
        .invoke_handler(tauri::generate_handler![
            get_wifi_status,
            toggle_wifi,
            scan_wifi,
            connect_wifi,
            get_wifi_config,
            set_wifi_config,
            get_bluetooth_status,
            toggle_bluetooth,
            scan_bluetooth,
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
            get_current_appearance_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
