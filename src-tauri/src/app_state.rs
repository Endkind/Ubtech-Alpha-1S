use std::path::PathBuf;
use std::sync::Mutex;
use crate::settings::config::Config as SettingsConfig;

#[derive(Debug)]
pub struct AppState {
    pub settings: Mutex<SettingsConfig>,
    pub config_path: PathBuf,
}

impl AppState {
    pub fn new(settings: SettingsConfig, config_path: PathBuf) -> Self {
        Self {
            settings: Mutex::new(settings),
            config_path,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Mutex::new(SettingsConfig::default()),
            config_path: PathBuf::from("config.toml"),
        }
    }
}
