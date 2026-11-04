use tauri::State;
use crate::AppState;
use crate::app_state::ResolutionChoice;
use crate::settings::config::{Config, ConfigConflict, ConfigRepairReport, UpdateConfig};

pub mod config;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Config, String> {
    let settings = state.settings.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    update: UpdateConfig,
    state: State<'_, AppState>,
) -> Result<Config, String> {
    let is_recovery = *state.is_recovery_mode.lock().await;
    let target_path = if is_recovery {
        &state.recovery_path
    } else {
        &state.config_path
    };

    // Work on a clone so the shared in-memory state remains unchanged on failure.
    let mut settings = {
        let guard = state.settings.lock().await;
        guard.clone()
    };

    // Apply update and validate on the clone — on validation error, return early and
    // leave the shared state untouched.
    settings.update(update).map_err(|err| err.to_string())?;

    // Save the modified clone. On IO error, the shared in-memory state is preserved.
    settings
        .save_to_file(target_path)
        .await
        .map_err(|err| format!("Failed to save configuration: {}", err))?;

    // Only after a successful write, update the shared in-memory state.
    {
        let mut guard = state.settings.lock().await;
        *guard = settings.clone();
    }

    Ok(settings)
}

#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<Config, String> {
    let is_recovery = *state.is_recovery_mode.lock().await;
    let target_path = if is_recovery {
        &state.recovery_path
    } else {
        &state.config_path
    };

    // Create the default config and save it before modifying the shared state.
    let default_config = Config::default();
    default_config
        .save_to_file(target_path)
        .await
        .map_err(|err| format!("Failed to save default configuration: {}", err))?;

    // Only after a successful write, update the shared in-memory state.
    {
        let mut guard = state.settings.lock().await;
        *guard = default_config.clone();
    }

    Ok(default_config)
}

#[tauri::command]
pub async fn get_config_conflict(state: State<'_, AppState>) -> Result<Option<ConfigConflict>, String> {
    let conflict = state.conflict.lock().await;
    Ok(conflict.clone())
}

#[tauri::command]
pub async fn resolve_config_conflict(
    choice: ResolutionChoice,
    state: State<'_, AppState>,
) -> Result<Config, String> {
    state.resolve_conflict(choice).await
}

#[tauri::command]
pub async fn is_recovery_mode(state: State<'_, AppState>) -> Result<bool, String> {
    let is_rec = state.is_recovery_mode.lock().await;
    Ok(*is_rec)
}

#[tauri::command]
pub async fn get_repair_report(state: State<'_, AppState>) -> Result<Option<ConfigRepairReport>, String> {
    let report = state.repair_report.lock().await;
    Ok(report.clone())
}

#[tauri::command]
pub async fn dismiss_repair_report(state: State<'_, AppState>) -> Result<(), String> {
    let mut report = state.repair_report.lock().await;
    *report = None;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::ResolutionChoice;
    use crate::settings::config::get_recovery_path;

    #[tokio::test]
    async fn test_save_settings_in_recovery_mode_writes_to_recovery_file() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_rec_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "AA:AA:AA:AA:AA:AA".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            true, // Recovery mode active
            None,
            None,
        );

        // Update settings while in recovery mode
        let mut settings = state.settings.lock().await;
        let update = UpdateConfig {
            mac_address: Some("BB:BB:BB:BB:BB:BB".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };
        settings.update(update).unwrap();

        let is_recovery = *state.is_recovery_mode.lock().await;
        let target_path = if is_recovery {
            &state.recovery_path
        } else {
            &state.config_path
        };
        settings.save_to_file(target_path).await.unwrap();
        drop(settings);

        // Verify that original config on disk is unchanged ("AA:AA:AA:AA:AA:AA")
        let loaded_orig = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded_orig.mac_address, "AA:AA:AA:AA:AA:AA");

        // Verify that recovery config on disk has the new mac ("BB:BB:BB:BB:BB:BB")
        let loaded_rec = Config::load_from_file(&recovery_path).await.unwrap();
        assert_eq!(loaded_rec.mac_address, "BB:BB:BB:BB:BB:BB");

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_conflict_resolution_updates_in_memory_and_removes_recovery_file() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_conflict_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut orig = Config::default();
        orig.mac_address = "00:11:22:33:44:55".to_string();
        orig.save_to_file(&config_path).await.unwrap();

        let mut rec = Config::default();
        rec.mac_address = "99:88:77:66:55:44".to_string();
        rec.save_to_file(&recovery_path).await.unwrap();

        let state = AppState::init(config_path.clone()).await;
        assert!(state.conflict.lock().await.is_some());

        let res = state.resolve_conflict(ResolutionChoice::Recovery).await.unwrap();
        assert_eq!(res.mac_address, "99:88:77:66:55:44");
        assert_eq!(state.settings.lock().await.mac_address, "99:88:77:66:55:44");
        assert!(!recovery_path.exists());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_save_settings_success_updates_ram_and_disk() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_save_ok_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "AA:AA:AA:AA:AA:AA".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = std::sync::Arc::new(AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            false,
            None,
            None,
        ));

        let update = UpdateConfig {
            mac_address: Some("BB:BB:BB:BB:BB:BB".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        // Simuliere die save_settings-Logik direkt (ohne Tauri-State-Wrapper)
        let is_recovery = *state.is_recovery_mode.lock().await;
        let target_path = if is_recovery {
            &state.recovery_path
        } else {
            &state.config_path
        };

        let mut settings = {
            let guard = state.settings.lock().await;
            guard.clone()
        };
        settings.update(update.clone()).unwrap();
        settings.save_to_file(target_path).await.unwrap();
        {
            let mut guard = state.settings.lock().await;
            *guard = settings.clone();
        }

        // RAM-State sollte aktualisiert sein
        assert_eq!(state.settings.lock().await.mac_address, "BB:BB:BB:BB:BB:BB");

        // Datei sollte aktualisiert sein
        let loaded = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded.mac_address, "BB:BB:BB:BB:BB:BB");

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_save_settings_validation_error_preserves_ram() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_save_val_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "AA:AA:AA:AA:AA:AA".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = std::sync::Arc::new(AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            false,
            None,
            None,
        ));

        // Ungültige MAC-Adresse — Validierung sollte fehlschlagen
        let update = UpdateConfig {
            mac_address: Some("invalid-mac".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        // Simuliere die save_settings-Logik: Klon, update (fehlschlägt), RAM bleibt erhalten
        let settings = {
            let guard = state.settings.lock().await;
            guard.clone()
        };
        let mut settings_clone = settings.clone();
        let update_result = settings_clone.update(update);
        assert!(update_result.is_err());

        // Da der Fehler auf dem Klon auftritt, wird der gemeinsame State NICHT überschrieben
        assert_eq!(state.settings.lock().await.mac_address, "AA:AA:AA:AA:AA:AA");

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_reset_settings_success_updates_ram_and_disk() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_reset_ok_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "AA:AA:AA:AA:AA:AA".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = std::sync::Arc::new(AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            false,
            None,
            None,
        ));

        // Simuliere die reset_settings-Logik direkt
        let is_recovery = *state.is_recovery_mode.lock().await;
        let target_path = if is_recovery {
            &state.recovery_path
        } else {
            &state.config_path
        };

        let default_config = Config::default();
        default_config.save_to_file(target_path).await.unwrap();
        {
            let mut guard = state.settings.lock().await;
            *guard = default_config.clone();
        }

        // RAM-State sollte auf Default zurückgesetzt sein
        let default_mac = Config::default().mac_address;
        assert_eq!(state.settings.lock().await.mac_address, default_mac);

        // Datei sollte auf Default zurückgesetzt sein
        let loaded = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded.mac_address, default_mac);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
