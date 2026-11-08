use tauri::Manager;
use toml;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::sync::Mutex;

pub struct ConfigState(pub Mutex<Config>);

// Config Structs
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Config {
    ollama: Ollama,
    paths: Paths,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Ollama {
    url: String,
    default_model: String,
    alternatives: u8,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Paths {
    item: Vec<Item>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
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
                item: vec!(Item {
                path: String::new(),
                default_language: "en".into()
                })
            }
        }
    }
}

// :reload_config

#[tauri::command]
pub fn reload_config(app: tauri::AppHandle, state: tauri::State<ConfigState>) -> Result<(), String> {
    let mut config = state.0.lock().unwrap();
    let config_dir = app.path().app_config_dir().map_err(|e| format!("Unable to load app config directory: {}", e))?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| format!("Could not create config directory: {}", e))?;
    }

    let config_path = config_dir.join("Config.toml");
    let new_conf = if config_path.exists() {
        let contents = fs::read_to_string(&config_path).map_err(|e| format!("Error reading file: {}", e))?;
        toml::from_str(&contents).unwrap_or_default()
    } else {
        let default_config = Config::default();
        let toml_string = toml::to_string(&default_config).map_err(|e| format!("Error parsing config into toml: {}", e))?;
        fs::write(&config_path, toml_string).map_err(|e| format!("Could not write default config to disk: {}", e))?;
        default_config
    };

    *config = new_conf.clone();

    //fs::write(config_path, &new_conf).map_err(|e| format!("Error writing changes to disk: {}", e))?;

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
