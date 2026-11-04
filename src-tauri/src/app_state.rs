use std::path::PathBuf;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::settings::config::{Config as SettingsConfig, ConfigConflict, ConfigRepairReport, get_recovery_path, init_config_state};

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
        }
    }

    pub async fn resolve_conflict(&self, choice: ResolutionChoice) -> Result<SettingsConfig, String> {
        let mut conflict_guard = self.conflict.lock().await;
        let mut settings_guard = self.settings.lock().await;
        let mut is_recovery_guard = self.is_recovery_mode.lock().await;

        let selected = match &*conflict_guard {
            Some(conflict) => match choice {
                ResolutionChoice::Original => conflict.original.clone(),
                ResolutionChoice::Recovery => conflict.recovery.clone(),
            },
            None => {
                if tokio::fs::try_exists(&self.recovery_path).await.unwrap_or(false) {
                    match choice {
                        ResolutionChoice::Original => settings_guard.clone(),
                        ResolutionChoice::Recovery => {
                            SettingsConfig::load_from_file_with_retry(&self.recovery_path)
                                .await
                                .map_err(|e| format!("Fehler beim Laden der Recovery-Konfiguration: {}", e))?
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
            .map_err(|err| format!("Fehler beim Speichern der ausgewählten Konfiguration: {}", err))?;

        // 2. Remove recovery file ONLY after saving succeeded
        if tokio::fs::try_exists(&self.recovery_path).await.unwrap_or(false) {
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
        }
    }
}
