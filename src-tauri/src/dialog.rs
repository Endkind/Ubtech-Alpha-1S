#[tauri::command]
pub async fn pick_file(
    default_path: Option<String>,
    filter_name: Option<String>,
    extensions: Option<Vec<String>>,
) -> Option<String> {
    let mut dialog = rfd::AsyncFileDialog::new();
    if let Some(ref path) = default_path
        && !path.is_empty()
    {
        dialog = dialog.set_directory(path);
    }
    if let (Some(name), Some(exts)) = (filter_name, extensions) {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(&name, &ext_refs);
    }
    let handle = dialog.pick_file().await;
    handle.map(|p| p.path().to_string_lossy().to_string())
}

#[tauri::command]
pub async fn pick_directory(default_path: Option<String>) -> Option<String> {
    let mut dialog = rfd::AsyncFileDialog::new();
    if let Some(ref path) = default_path
        && !path.is_empty()
    {
        dialog = dialog.set_directory(path);
    }
    let handle = dialog.pick_folder().await;
    handle.map(|p| p.path().to_string_lossy().to_string())
}
