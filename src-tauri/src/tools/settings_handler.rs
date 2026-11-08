use toml::Table;
use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::io::ErrorKind::NotFound;

// User structs /user/Settings.toml
#[derive(Debug, Deserialize)]
struct Config {
    ollama: Ollama,
    paths: Paths,
}

#[derive(Debug, Deserialize)]
struct Ollama {
    url: String,
    default_model: String,
    alternatives: u8,
}

#[derive(Debug, Deserialize)]
struct Paths {
    item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    path: String,
    default_language: String,
}

// :reload_config

#[tauri::command]
pub fn load_config() -> Result<(), String> {
   // let settings_file: String = read_to_string("../../../user/Settings.toml")
   //     .map_err(|e| format!("Error: {}", e))?.into();

    let settings_file: String = match fs::read_to_string("../../../user/Settings.toml") {
        Ok (contents) => {
            contents.into()
        }
        Err(e) => {
            if e.kind() == NotFound {
                println!("Config not found!");
                panic!();
            }
            String::new()
        }
    };

    if let Ok(config) = toml::from_str::<Config>(&settings_file) {
        print!("Config loaded: {:?}", config);
    } else {
        print!("Error loading config.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_conf() {
        let _ = load_config();
    }
}
