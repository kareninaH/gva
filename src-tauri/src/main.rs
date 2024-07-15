// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name == "".to_string() {
        return Err("参数不能为空".into());
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
fn tauri_version() -> String {
    "当前系统版本为: 1.0".to_string()
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, tauri_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
