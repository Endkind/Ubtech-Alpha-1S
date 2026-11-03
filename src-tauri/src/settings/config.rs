use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validator::regex::MAC_ADDRESS as MAC_ADDRESS_REGEX;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateConfig {
    pub mac_address: Option<String>,
    pub default_delay: Option<usize>,
    pub default_action_duration: Option<usize>,
    pub python_sdk: Option<String>,
    pub workdir: Option<String>,
    pub venv: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Config {
    #[validate(regex(path = *MAC_ADDRESS_REGEX, message = "Bitte eine gültige MAC-Adresse eingeben (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)"))]
    pub mac_address: String,

    #[validate(range(min = 0, max = 2000, message = "Standard-Delay muss zwischen 0 und 2000 ms liegen"))]
    pub default_delay: usize,

    #[validate(range(min = 10, max = 200, message = "Standarddauer für Aktionen muss zwischen 10 und 200 ms liegen"))]
    pub default_action_duration: usize,

    pub python_sdk: String,
    pub workdir: String,
    pub venv: String,
}

impl Config {
    pub fn update(&mut self, update: UpdateConfig) -> Result<(), validator::ValidationErrors> {
        let mut new_config = self.clone();

        if let Some(mac) = update.mac_address {
            new_config.mac_address = mac;
        }

        if let Some(delay) = update.default_delay {
            new_config.default_delay = delay;
        }

        if let Some(duration) = update.default_action_duration {
            new_config.default_action_duration = duration;
        }

        if let Some(sdk) = update.python_sdk {
            new_config.python_sdk = sdk;
        }

        if let Some(workdir) = update.workdir {
            new_config.workdir = workdir;
        }

        if let Some(venv) = update.venv {
            new_config.venv = venv;
        }

        new_config.validate()?;
        *self = new_config;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mac_address: "00:11:22:33:44:55".to_owned(),
            default_delay: 500,
            default_action_duration: 50,
            python_sdk: String::new(),
            workdir: String::new(),
            venv: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_full_update() {
        let mut config = Config::default();

        let update = UpdateConfig {
            mac_address: Some("AA:BB:CC:DD:EE:FF".to_string()),
            default_delay: Some(1000),
            default_action_duration: Some(120),
            python_sdk: Some("C:\\Projekte\\Ubtech-Alpha-1S-SDK\\main.py".to_string()),
            workdir: Some("C:\\Projekte\\Ubtech-Alpha-1S-SDK".to_string()),
            venv: Some("C:\\Projekte\\Ubtech-Alpha-1S-SDK\\.venv".to_string()),
        };

        assert!(config.update(update).is_ok());
        assert_eq!(config.mac_address, "AA:BB:CC:DD:EE:FF");
        assert_eq!(config.default_delay, 1000);
        assert_eq!(config.default_action_duration, 120);
        assert_eq!(config.python_sdk, "C:\\Projekte\\Ubtech-Alpha-1S-SDK\\main.py");
        assert_eq!(config.workdir, "C:\\Projekte\\Ubtech-Alpha-1S-SDK");
        assert_eq!(config.venv, "C:\\Projekte\\Ubtech-Alpha-1S-SDK\\.venv");
    }

    #[test]
    fn test_valid_partial_update() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();

        let update = UpdateConfig {
            mac_address: None,
            default_delay: Some(200),
            default_action_duration: Some(80),
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        assert!(config.update(update).is_ok());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.default_delay, 200);
        assert_eq!(config.default_action_duration, 80);
    }

    #[test]
    fn test_valid_hyphen_mac_update() {
        let mut config = Config::default();

        let update = UpdateConfig {
            mac_address: Some("AA-BB-CC-DD-EE-FF".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        assert!(config.update(update).is_ok());
        assert_eq!(config.mac_address, "AA-BB-CC-DD-EE-FF");
    }

    #[test]
    fn test_invalid_mac_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();
        let initial_delay = config.default_delay;

        let update = UpdateConfig {
            mac_address: Some("invalid-mac".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        let result = config.update(update);
        assert!(result.is_err());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.default_delay, initial_delay);
    }

    #[test]
    fn test_mixed_separator_mac_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();
        let initial_delay = config.default_delay;

        let update = UpdateConfig {
            mac_address: Some("00:11-22:33-44:55".to_string()),
            default_delay: None,
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        let result = config.update(update);
        assert!(result.is_err());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.default_delay, initial_delay);
    }

    #[test]
    fn test_invalid_delay_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();
        let initial_delay = config.default_delay;

        let update = UpdateConfig {
            mac_address: None,
            default_delay: Some(2500),
            default_action_duration: None,
            python_sdk: None,
            workdir: None,
            venv: None,
        };

        let result = config.update(update);
        assert!(result.is_err());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.default_delay, initial_delay);
    }

    #[test]
    fn test_invalid_action_duration_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_duration = config.default_action_duration;

        let update_too_low = UpdateConfig {
            mac_address: None,
            default_delay: None,
            default_action_duration: Some(5),
            python_sdk: None,
            workdir: None,
            venv: None,
        };
        assert!(config.update(update_too_low).is_err());
        assert_eq!(config.default_action_duration, initial_duration);

        let update_too_high = UpdateConfig {
            mac_address: None,
            default_delay: None,
            default_action_duration: Some(250),
            python_sdk: None,
            workdir: None,
            venv: None,
        };
        assert!(config.update(update_too_high).is_err());
        assert_eq!(config.default_action_duration, initial_duration);
    }
}
