pub mod settings;
pub mod validator;
mod app_state;

use settings::{get_settings, save_settings};
use crate::app_state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, get_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
