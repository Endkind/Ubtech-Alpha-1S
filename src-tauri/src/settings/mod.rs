use tauri::State;
use crate::AppState;
use crate::settings::config::{Config, UpdateConfig};

pub mod config;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Config, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    update: UpdateConfig,
    state: State<'_, AppState>,
) -> Result<Config, String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;

    settings.update(update).map_err(|err| err.to_string())?;

    settings
        .save_to_file(&state.config_path)
        .map_err(|err| format!("Fehler beim Speichern der Konfiguration: {}", err))?;

    Ok(settings.clone())
}

#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<Config, String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
    let default_config = Config::default();
    *settings = default_config.clone();

    settings
        .save_to_file(&state.config_path)
        .map_err(|err| format!("Fehler beim Speichern der Standardkonfiguration: {}", err))?;

    Ok(default_config)
}
