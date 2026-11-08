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
            let config_dir = app.path().app_config_dir()?;

            if !config_dir.exists(){
                fs::create_dir_all(&config_dir)?;
            }

            let config_path = config_dir.join("Config.toml");

            let config = if config_path.exists() {
                let contents = fs::read_to_string(&config_path)?;
                toml::from_str(&contents).unwrap_or_default()
            } else {
                let default_config = settings_handler::Config::default();
                let toml_string = toml::to_string(&default_config)?;
                fs::write(&config_path, toml_string)?;
                default_config
            };

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
