//! SoraWorks Launcher backend (Tauri v2 library crate).

mod auth;
mod commands;
mod error;
mod http;
mod java;
mod launch;
mod mojang;
mod paths;

use launch::LaunchState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .manage(LaunchState::default())
        .invoke_handler(tauri::generate_handler![
            commands::auth_login_browser,
            commands::auth_start_device_code,
            commands::auth_complete_device_code,
            commands::accounts_list,
            commands::accounts_remove,
            commands::accounts_set_active,
            commands::auth_refresh,
            commands::mojang_versions,
            commands::java_ensure,
            commands::launch_vanilla,
            commands::launch_stop,
            commands::launch_is_running,
            commands::paths_default_game_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
