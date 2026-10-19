#[tauri::command]
pub fn save_title(title: &str) -> String {
    format!("Title: {}", title)
}
