use tauri::State;
use crate::AppState;
use crate::settings::config::{Config, UpdateConfig};

pub mod config;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<Config, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
pub fn save_settings(
    update: UpdateConfig,
    state: State<'_, AppState>,
) -> Result<Config, String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;

    settings.update(update).map_err(|err| err.to_string())?;

    Ok(settings.clone())
}
