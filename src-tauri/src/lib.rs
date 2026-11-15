// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod tools;

use std::sync::Mutex;
use tauri::Manager;

use commands::edit::{ save_title, load_file, save_file, get_new_filepath, rename_file };
use tools::ollama_handler::ollama_list;
use tools::settings_handler::{ reload_config, add_path, load_config, ConfigState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        .setup(|app| {
            let config = load_config(app)?;
            app.manage(ConfigState(Mutex::new(config)));
            Ok(())

        })

        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_title,
            load_file,
            save_file,
            get_new_filepath,
            rename_file,
            ollama_list,
            reload_config,
            add_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
