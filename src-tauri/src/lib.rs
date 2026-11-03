pub mod settings;
pub mod validator;
pub mod dialog;
pub mod platform;
pub mod path;
mod app_state;

use settings::{get_settings, save_settings, reset_settings};
use settings::config::Config;
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
            std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
            let config_path = config_dir.join("config.toml");
            let config = Config::load_or_init(&config_path);

            app.manage(AppState::new(config, config_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_settings,
            save_settings,
            reset_settings,
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
