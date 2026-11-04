use crate::settings::config::{
    Config as SettingsConfig, ConfigConflict, ConfigRepairReport, UpdateConfig, get_recovery_path,
    init_config_state,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolutionChoice {
    Original,
    Recovery,
}

#[derive(Debug)]
pub struct AppState {
    pub settings: Mutex<SettingsConfig>,
    pub config_path: PathBuf,
    pub recovery_path: PathBuf,
    pub is_recovery_mode: Mutex<bool>,
    pub conflict: Mutex<Option<ConfigConflict>>,
    pub repair_report: Mutex<Option<ConfigRepairReport>>,
    /// Serializes all persistence operations (save, reset, conflict resolution)
    /// so that RAM state and TOML file remain consistent after each operation.
    pub persistence_lock: Mutex<()>,
}

impl AppState {
    pub fn new(
        settings: SettingsConfig,
        config_path: PathBuf,
        recovery_path: PathBuf,
        is_recovery_mode: bool,
        conflict: Option<ConfigConflict>,
        repair_report: Option<ConfigRepairReport>,
    ) -> Self {
        Self {
            settings: Mutex::new(settings),
            config_path,
            recovery_path,
            is_recovery_mode: Mutex::new(is_recovery_mode),
            conflict: Mutex::new(conflict),
            repair_report: Mutex::new(repair_report),
            persistence_lock: Mutex::new(()),
        }
    }

    pub async fn init(config_path: PathBuf) -> Self {
        let recovery_path = get_recovery_path(&config_path);
        let init_res = init_config_state(&config_path).await;
        Self {
            settings: Mutex::new(init_res.config),
            config_path,
            recovery_path,
            is_recovery_mode: Mutex::new(init_res.is_recovery_mode),
            conflict: Mutex::new(init_res.conflict),
            repair_report: Mutex::new(init_res.repair_report),
            persistence_lock: Mutex::new(()),
        }
    }

    pub async fn resolve_conflict(
        &self,
        choice: ResolutionChoice,
    ) -> Result<SettingsConfig, String> {
        // Persistence-Lock serialisiert diesen Vorgang mit save_settings / reset_settings.
        let _persist = self.persistence_lock.lock().await;
        let mut conflict_guard = self.conflict.lock().await;
        let mut settings_guard = self.settings.lock().await;
        let mut is_recovery_guard = self.is_recovery_mode.lock().await;

        let selected = match &*conflict_guard {
            Some(conflict) => match choice {
                ResolutionChoice::Original => conflict.original.clone(),
                ResolutionChoice::Recovery => conflict.recovery.clone(),
            },
            None => {
                if tokio::fs::try_exists(&self.recovery_path)
                    .await
                    .unwrap_or(false)
                {
                    match choice {
                        ResolutionChoice::Original => settings_guard.clone(),
                        ResolutionChoice::Recovery => {
                            SettingsConfig::load_from_file_with_retry(&self.recovery_path)
                                .await
                                .map_err(|e| {
                                    format!("Fehler beim Laden der Recovery-Konfiguration: {}", e)
                                })?
                        }
                    }
                } else {
                    return Ok(settings_guard.clone());
                }
            }
        };

        // 1. Atomically save selected configuration to config_path
        selected
            .save_to_file(&self.config_path)
            .await
            .map_err(|err| {
                format!(
                    "Fehler beim Speichern der ausgewählten Konfiguration: {}",
                    err
                )
            })?;

        // 2. Remove recovery file ONLY after saving succeeded
        if tokio::fs::try_exists(&self.recovery_path)
            .await
            .unwrap_or(false)
        {
            tokio::fs::remove_file(&self.recovery_path)
                .await
                .map_err(|err| format!("Fehler beim Löschen der Recovery-Datei: {}", err))?;
        }

        // 3. Update in-memory state
        *settings_guard = selected.clone();
        *is_recovery_guard = false;
        *conflict_guard = None;

        Ok(selected)
    }

    /// Saves the updated configuration and updates the in-memory state.
    /// The persistence lock serializes this operation with other save operations
    /// so that RAM state and TOML file remain consistent after completion.
    pub async fn save_settings(&self, update: UpdateConfig) -> Result<SettingsConfig, String> {
        let _persist = self.persistence_lock.lock().await;

        let is_recovery = *self.is_recovery_mode.lock().await;
        let target_path = if is_recovery {
            &self.recovery_path
        } else {
            &self.config_path
        };

        // Work on a clone so the shared in-memory state remains unchanged on failure.
        let mut settings = {
            let guard = self.settings.lock().await;
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
            let mut guard = self.settings.lock().await;
            *guard = settings.clone();
        }

        Ok(settings)
    }

    /// Resets the configuration to default values and updates the in-memory state.
    /// The persistence lock serializes this operation with other save operations.
    pub async fn reset_settings(&self) -> Result<SettingsConfig, String> {
        let _persist = self.persistence_lock.lock().await;

        let is_recovery = *self.is_recovery_mode.lock().await;
        let target_path = if is_recovery {
            &self.recovery_path
        } else {
            &self.config_path
        };

        // Create the default config and save it before modifying the shared state.
        let default_config = SettingsConfig::default();
        default_config
            .save_to_file(target_path)
            .await
            .map_err(|err| format!("Failed to save default configuration: {}", err))?;

        // Only after a successful write, update the shared in-memory state.
        {
            let mut guard = self.settings.lock().await;
            *guard = default_config.clone();
        }

        Ok(default_config)
    }
}

impl Default for AppState {
    fn default() -> Self {
        let config_path = PathBuf::from("config.toml");
        let recovery_path = get_recovery_path(&config_path);
        Self {
            settings: Mutex::new(SettingsConfig::default()),
            config_path,
            recovery_path,
            is_recovery_mode: Mutex::new(false),
            conflict: Mutex::new(None),
            repair_report: Mutex::new(None),
            persistence_lock: Mutex::new(()),
        }
    }
}
