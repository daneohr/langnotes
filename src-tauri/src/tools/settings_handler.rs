use tauri::utils::config::AppConfig;
use toml::Table;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::error::Error;
use std::io::ErrorKind::NotFound;

// User structs /user/Settings.toml
#[derive(Serialize Debug, Deserialize)]
pub struct Config {
    ollama: Ollama,
    paths: Paths,
}

#[derive(Serialize Debug, Deserialize)]
struct Ollama {
    url: String,
    default_model: String,
    alternatives: u8,
}

#[derive(Serialize Debug, Deserialize)]
struct Paths {
    item: Vec<Item>,
}

#[derive(Serialize Debug, Deserialize)]
struct Item {
    path: String,
    default_language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama: Ollama {
                url: "http://localhost:11434".into(),
                default_model: String::new(),
                alternatives: 3,
            },
            paths: Paths {
                item: Vec::from(Item {
                path: String::new(),
                default_language: "en".into()
                })
            }
        }
    }
}

// :reload_config

#[tauri::command]
pub fn load_config() -> Result<(), String> {

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
