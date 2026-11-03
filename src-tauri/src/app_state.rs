use std::sync::Mutex;
use crate::settings::config::Config as SettingsConfig;

#[derive(Debug, Default)]
pub struct AppState {
    pub settings: Mutex<SettingsConfig>
}
