use std::sync::LazyLock;
use regex::Regex;

pub static MAC_ADDRESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}|([0-9A-Fa-f]{2}-){5}[0-9A-Fa-f]{2})$").unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_valid_colon() {
        assert!(MAC_ADDRESS.is_match("00:11:22:33:44:55"));
        assert!(MAC_ADDRESS.is_match("AA:BB:CC:DD:EE:FF"));
        assert!(MAC_ADDRESS.is_match("a1:b2:c3:d4:e5:f6"));
    }

    #[test]
    fn test_mac_address_valid_hyphen() {
        assert!(MAC_ADDRESS.is_match("00-11-22-33-44-55"));
        assert!(MAC_ADDRESS.is_match("AA-BB-CC-DD-EE-FF"));
        assert!(MAC_ADDRESS.is_match("a1-b2-c3-d4-e5-f6"));
    }

    #[test]
    fn test_mac_address_rejects_mixed_separators() {
        assert!(!MAC_ADDRESS.is_match("00:11-22:33-44:55"));
        assert!(!MAC_ADDRESS.is_match("00-11:22:33:44:55"));
        assert!(!MAC_ADDRESS.is_match("00:11:22:33:44-55"));
        assert!(!MAC_ADDRESS.is_match("00:11:22-33:44:55"));
    }

    #[test]
    fn test_mac_address_rejects_invalid_format() {
        assert!(!MAC_ADDRESS.is_match("invalid-mac"));
        assert!(!MAC_ADDRESS.is_match("00:11:22:33:44"));
        assert!(!MAC_ADDRESS.is_match("00:11:22:33:44:55:66"));
        assert!(!MAC_ADDRESS.is_match("00:11:22:33:44:5G"));
    }
}
