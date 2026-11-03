#[tauri::command]
pub fn get_family() -> &'static str {
    std::env::consts::FAMILY
}
