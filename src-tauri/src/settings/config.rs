use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validator::regex::MAC_ADDRESS as MAC_ADDRESS_REGEX;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateConfig {
    pub mac_address: Option<String>,
    pub delay: Option<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Config {
    #[validate(regex(path = *MAC_ADDRESS_REGEX, message = "Bitte eine gültige MAC-Adresse eingeben (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)"))]
    pub mac_address: String,

    #[validate(range(min = 0, max = 2000, message = "Delay muss zwischen 0 und 2000 ms liegen"))]
    pub delay: usize,
}

impl Config {
    pub fn update(&mut self, update: UpdateConfig) -> Result<(), validator::ValidationErrors> {
        let mut new_config = self.clone();

        if let Some(mac) = update.mac_address {
            new_config.mac_address = mac;
        }

        if let Some(delay) = update.delay {
            new_config.delay = delay;
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
            delay: 500
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
            delay: Some(1000),
        };

        assert!(config.update(update).is_ok());
        assert_eq!(config.mac_address, "AA:BB:CC:DD:EE:FF");
        assert_eq!(config.delay, 1000);
    }

    #[test]
    fn test_valid_partial_update() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();

        let update = UpdateConfig {
            mac_address: None,
            delay: Some(200),
        };

        assert!(config.update(update).is_ok());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.delay, 200);
    }

    #[test]
    fn test_invalid_mac_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();
        let initial_delay = config.delay;

        let update = UpdateConfig {
            mac_address: Some("invalid-mac".to_string()),
            delay: None,
        };

        let result = config.update(update);
        assert!(result.is_err());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.delay, initial_delay);
    }

    #[test]
    fn test_invalid_delay_does_not_mutate_config() {
        let mut config = Config::default();
        let initial_mac = config.mac_address.clone();
        let initial_delay = config.delay;

        let update = UpdateConfig {
            mac_address: None,
            delay: Some(2500),
        };

        let result = config.update(update);
        assert!(result.is_err());
        assert_eq!(config.mac_address, initial_mac);
        assert_eq!(config.delay, initial_delay);
    }
}
