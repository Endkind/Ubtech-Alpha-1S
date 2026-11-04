pub mod settings;
pub mod validator;
pub mod dialog;
pub mod platform;
pub mod path;
pub mod app_state;

use settings::{
    get_settings, save_settings, reset_settings,
    get_config_conflict, resolve_config_conflict, is_recovery_mode,
    get_repair_report, dismiss_repair_report,
};
use dialog::{pick_file, pick_directory};
use platform::get_family;
use path::{path_dirname, path_join, path_basename, path_is_absolute};
use crate::app_state::AppState;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
            let config_path = config_dir.join("config.toml");
            let app_state = tauri::async_runtime::block_on(async {
                tokio::fs::create_dir_all(&config_dir)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok::<_, String>(AppState::init(config_path).await)
            })?;

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_settings,
            save_settings,
            reset_settings,
            get_config_conflict,
            resolve_config_conflict,
            is_recovery_mode,
            get_repair_report,
            dismiss_repair_report,
            pick_file,
            pick_directory,
            get_family,
            path_dirname,
            path_join,
            path_basename,
            path_is_absolute
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
