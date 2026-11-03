use std::path::Path;
use std::process::exit;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use validator::Validate;
use crate::validator::regex::MAC_ADDRESS as MAC_ADDRESS_REGEX;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateConfig {
    pub mac_address: Option<String>,
    pub default_delay: Option<usize>,
    pub default_action_duration: Option<usize>,
    pub python_sdk: Option<String>,
    pub workdir: Option<String>,
    pub venv: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.validate()?;
        let path = path.as_ref();
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        tokio::fs::create_dir_all(parent).await?;

        let toml_str = toml::to_string_pretty(self)?;

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("config.toml");

        let temp_file_name = format!(
            ".{}.tmp.{}_{}_{}",
            file_name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let temp_path = parent.join(temp_file_name);

        let mut temp_file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .await?;

        let write_result: Result<(), std::io::Error> = async {
            temp_file.write_all(toml_str.as_bytes()).await?;
            temp_file.flush().await?;
            temp_file.sync_all().await?;
            Ok(())
        }
        .await;

        drop(temp_file);

        if let Err(e) = write_result {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(Box::new(e));
        }

        if let Err(e) = tokio::fs::rename(&temp_path, path).await {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(Box::new(e));
        }

        Ok(())
    }

    pub async fn load_or_init<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        match tokio::fs::try_exists(path).await {
            Ok(true) => {
                match Self::load_from_file(path).await {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!(
                            "Fehler beim Laden der Konfiguration von {}: {}. Standardwerte werden verwendet.",
                            path.display(),
                            err
                        );
                        Self::default()
                    }
                }
            }
            Ok(false) => {
                let default_config = Self::default();
                if let Err(err) = default_config.save_to_file(path).await {
                    eprintln!(
                        "Fehler beim Erstellen der Standardkonfiguration unter {}: {}",
                        path.display(),
                        err
                    );
                }
                default_config
            }
            Err(err) => {
                eprintln!(
                    "Fehler beim Überprüfen der Konfiguration unter {}: {}. Standardwerte werden verwendet.",
                    path.display(),
                    err
                );
                Self::default()
            }
        }
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

    #[tokio::test]
    async fn test_save_and_load_file_roundtrip() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("subdir").join("config.toml");

        let mut config = Config::default();
        config.mac_address = "11:22:33:44:55:66".to_string();
        config.default_delay = 750;
        config.default_action_duration = 100;
        config.python_sdk = "/path/to/sdk.py".to_string();
        config.workdir = "/path/to/workdir".to_string();
        config.venv = "/path/to/venv".to_string();

        assert!(config.save_to_file(&config_path).await.is_ok());
        assert!(config_path.exists());

        let loaded = Config::load_from_file(&config_path).await.expect("failed to load config");
        assert_eq!(loaded, config);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_load_from_file_invalid_toml() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_inv_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        tokio::fs::write(&config_path, "invalid toml content = ==").await.unwrap();

        assert!(Config::load_from_file(&config_path).await.is_err());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_load_from_file_invalid_validation() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_val_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        let invalid_content = r#"
mac_address = "invalid_mac"
default_delay = 500
default_action_duration = 50
python_sdk = ""
workdir = ""
venv = ""
"#;
        tokio::fs::write(&config_path, invalid_content).await.unwrap();

        assert!(Config::load_from_file(&config_path).await.is_err());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_load_or_init_creates_default_when_missing() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_init_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");

        let loaded = Config::load_or_init(&config_path).await;
        assert_eq!(loaded, Config::default());
        assert!(config_path.exists());

        let loaded_again = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded_again, Config::default());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_load_or_init_preserves_corrupted_file_and_does_not_overwrite() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_corrupt_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        let corrupted_content = "corrupted = true [[[invalid";
        tokio::fs::write(&config_path, corrupted_content).await.unwrap();

        let loaded = Config::load_or_init(&config_path).await;
        assert_eq!(loaded, Config::default());

        // Ensure the corrupted file was NOT overwritten with default values
        let file_content = tokio::fs::read_to_string(&config_path).await.unwrap();
        assert_eq!(file_content, corrupted_content);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_failed_write_preserves_previous_configuration() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_failed_write_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");

        let mut valid_config = Config::default();
        valid_config.mac_address = "AA:BB:CC:DD:EE:FF".to_string();
        valid_config.default_delay = 800;
        assert!(valid_config.save_to_file(&config_path).await.is_ok());

        // Attempt to save an invalid configuration that fails validation
        let invalid_config = Config {
            mac_address: "invalid-mac".to_string(),
            default_delay: 500,
            default_action_duration: 50,
            python_sdk: String::new(),
            workdir: String::new(),
            venv: String::new(),
        };

        assert!(invalid_config.save_to_file(&config_path).await.is_err());

        // Verify the original valid configuration on disk is intact
        let loaded = Config::load_from_file(&config_path).await.unwrap();
        assert_eq!(loaded, valid_config);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_concurrent_writes_do_not_corrupt_file() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_concurrent_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");

        let mut handles = Vec::new();
        for i in 0..50 {
            let path = config_path.clone();
            let handle = tokio::spawn(async move {
                let mut config = Config::default();
                config.default_delay = (i % 2000) as usize;
                config.default_action_duration = 50 + (i % 50) as usize;
                config.workdir = format!("/workdir/task_{}", i);
                config.save_to_file(&path).await
            });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.await.expect("task panicked");
            assert!(res.is_ok());
        }

        // Verify that the final file is valid TOML and deserializes cleanly
        let loaded = Config::load_from_file(&config_path).await;
        assert!(loaded.is_ok(), "Config on disk corrupted by concurrent writes: {:?}", loaded.err());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_persistence_across_restarts() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_restart_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");

        let mut initial_config = Config::default();
        initial_config.mac_address = "12:34:56:78:9A:BC".to_string();
        initial_config.default_delay = 1200;
        initial_config.default_action_duration = 150;
        initial_config.python_sdk = "/usr/bin/python3".to_string();
        initial_config.workdir = "/app/workdir".to_string();
        initial_config.venv = "/app/venv".to_string();

        assert!(initial_config.save_to_file(&config_path).await.is_ok());

        // Simulate app restart by calling load_or_init on the existing path
        let restarted_config = Config::load_or_init(&config_path).await;
        assert_eq!(restarted_config, initial_config);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_atomic_write_leaves_no_temp_files() {
        let temp_dir = std::env::temp_dir().join(format!("ubtech_test_notemp_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let config_path = temp_dir.join("config.toml");

        let config = Config::default();
        assert!(config.save_to_file(&config_path).await.is_ok());

        let mut read_dir = tokio::fs::read_dir(&temp_dir).await.unwrap();
        let mut file_names = Vec::new();
        while let Some(entry) = read_dir.next_entry().await.unwrap() {
            file_names.push(entry.file_name().to_string_lossy().to_string());
        }

        assert_eq!(file_names, vec!["config.toml"]);

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
