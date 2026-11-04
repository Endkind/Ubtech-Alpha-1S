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
    state.save_settings(update).await
}

#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<Config, String> {
    state.reset_settings().await
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

        // Call the production save_settings method
        let update = UpdateConfig {
            mac_address: Some("BB:BB:BB:BB:BB:BB".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };
        let result = state.save_settings(update).await.unwrap();
        assert_eq!(result.mac_address, "BB:BB:BB:BB:BB:BB");

        // Verify that original config on disk is unchanged ("AA:AA:AA:AA:AA:AA")
        let loaded_orig = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded_orig.mac_address, "AA:AA:AA:AA:AA:AA");

        // Verify that recovery config on disk has the new mac ("BB:BB:BB:BB:BB:BB")
        let loaded_rec = Config::load_from_file(&recovery_path).await.unwrap();
        assert_eq!(loaded_rec.mac_address, "BB:BB:BB:BB:BB:BB");

        // Verify RAM state is updated
        assert_eq!(state.settings.lock().await.mac_address, "BB:BB:BB:BB:BB:BB");

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

        // Call the production save_settings method
        let result = state.save_settings(update).await.unwrap();
        assert_eq!(result.mac_address, "BB:BB:BB:BB:BB:BB");

        // RAM state should be updated
        assert_eq!(state.settings.lock().await.mac_address, "BB:BB:BB:BB:BB:BB");

        // File should be updated
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

        // Invalid MAC address — validation should fail
        let update = UpdateConfig {
            mac_address: Some("invalid-mac".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        // Call the production save_settings method — should return error
        let result = state.save_settings(update).await;
        assert!(result.is_err());

        // RAM state should remain unchanged
        assert_eq!(state.settings.lock().await.mac_address, "AA:AA:AA:AA:AA:AA");

        // File should remain unchanged
        let loaded = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded.mac_address, "AA:AA:AA:AA:AA:AA");

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

        // Call the production reset_settings method
        let result = state.reset_settings().await.unwrap();
        let default_mac = Config::default().mac_address;
        assert_eq!(result.mac_address, default_mac);

        // RAM state should be reset to default
        assert_eq!(state.settings.lock().await.mac_address, default_mac);

        // File should be reset to default
        let loaded = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded.mac_address, default_mac);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    /// Tests that parallel save_settings calls are serialized and
    /// RAM status and TOML file remain consistent after all calls complete.
    #[tokio::test]
    async fn test_concurrent_save_settings_serialized() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_concurrent_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "00:00:00:00:00:00".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = std::sync::Arc::new(AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            false,
            None,
            None,
        ));

        // Parallele save_settings-Aufrufe über die production method.
        let mut handles = vec![];
        for i in 0..10u32 {
            let state = state.clone();
            let handle = tokio::spawn(async move {
                let mac = format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", i, i, i, i, i, i);
                let update = UpdateConfig {
                    mac_address: Some(mac),
                    default_delay: None,
                    default_action_duration: None,
                    python_sdk: None,
                    workdir: None,
                    venv: None,
                };

                // Call the production save_settings method (includes persistence_lock)
                state.save_settings(update).await
            });
            handles.push(handle);
        }

        // Wait for all calls — tokio::spawn returns Result<T, JoinError>.
        let results: Vec<Result<Result<Config, String>, _>> = futures::future::join_all(handles).await;

        // All calls should be successful.
        let success_count = results.iter().filter(|r| matches!(r, Ok(Ok(_)))).count();
        assert!(success_count > 0, "At least one save_settings call should succeed");

        // After all calls complete, RAM and file must have the same MAC value.
        let ram_mac = {
            let guard = state.settings.lock().await;
            guard.mac_address.clone()
        };
        let disk_mac = Config::load_from_file(&config_path).await.unwrap().mac_address;

        assert_eq!(
            ram_mac, disk_mac,
            "RAM status and TOML file must be consistent after parallel save_settings calls"
        );

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    /// Tests that save_settings and reset_settings are not executed concurrently
    /// and RAM and file remain consistent after completion.
    #[tokio::test]
    async fn test_save_and_reset_serialized() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_mod_save_reset_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        let recovery_path = get_recovery_path(&config_path);

        let mut initial_config = Config::default();
        initial_config.mac_address = "00:00:00:00:00:00".to_string();
        initial_config.save_to_file(&config_path).await.unwrap();

        let state = std::sync::Arc::new(AppState::new(
            initial_config.clone(),
            config_path.clone(),
            recovery_path.clone(),
            false,
            None,
            None,
        ));

        // Parallele save_settings- und reset_settings-Aufrufe über production methods.
        let mut handles = vec![];

        for i in 0..5u32 {
            let state = state.clone();
            let handle = tokio::spawn(async move {
                let mac = format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", i, i, i, i, i, i);
                let update = UpdateConfig {
                    mac_address: Some(mac),
                    default_delay: None,
                    default_action_duration: None,
                    python_sdk: None,
                    workdir: None,
                    venv: None,
                };

                // Call the production save_settings method (includes persistence_lock)
                state.save_settings(update).await.map(|_| ())
            });
            handles.push(handle);
        }

        for _ in 0..3 {
            let state = state.clone();
            let handle = tokio::spawn(async move {
                // Call the production reset_settings method (includes persistence_lock)
                state.reset_settings().await.map(|_| ())
            });
            handles.push(handle);
        }

        let _ = futures::future::join_all(handles).await;

        // After all calls complete, RAM and file must be consistent.
        let ram_mac = {
            let guard = state.settings.lock().await;
            guard.mac_address.clone()
        };
        let disk_mac = Config::load_from_file(&config_path).await.unwrap().mac_address;

        assert_eq!(
            ram_mac, disk_mac,
            "RAM status and TOML file must be consistent after mixed save/reset calls"
        );

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
