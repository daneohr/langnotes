// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod tools;

use std::fs;
use std::sync::Mutex;
use tauri::Manager;
use toml;

use commands::edit::{ save_title, load_file, save_file, get_new_filepath, rename_file };
use tools::ollama_handler::ollama_list;
use tools::settings_handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        .setup(|app| {
            let config = settings_handler::load_config(app.handle())?;
            app.manage(settings_handler::ConfigState(Mutex::new(config)));
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
            settings_handler::reload_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
