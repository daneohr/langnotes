use tauri::Manager;
use toml;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::sync::Mutex;
use std::error;

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

// load config from file or create it
pub fn load_config(app: &tauri::AppHandle) -> Result<Config, Box<dyn error::Error>> {
    let config_dir = app.path().app_config_dir()?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("Config.toml");
    let config = if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;
        toml::from_str(&contents).unwrap_or_default()
    } else {
        let default_config = Config::default();
        let toml_string = toml::to_string(&default_config)?;
        fs::write(&config_path, toml_string)?;
        default_config
    };

    Ok(config)
}

// :reload_config
#[tauri::command]
pub fn reload_config(app: tauri::AppHandle, state: tauri::State<ConfigState>) -> Result<(), String> {
    // Mutex of config, which is stored in memory (we want to update it)
    let mut config = state.0.lock().unwrap();
    let new_conf = load_config(&app).map_err(|e| format!("Error loading config: {}", e))?;

    *config = new_conf.clone();

    Ok(())
}
