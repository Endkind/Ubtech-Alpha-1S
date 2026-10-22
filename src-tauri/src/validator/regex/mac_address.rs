use std::sync::LazyLock;
use regex::Regex;

pub static MAC_ADDRESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap()
});
