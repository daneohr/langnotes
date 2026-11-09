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
pub fn load_config<R: tauri::Runtime, M: Manager<R>>(app: &M) -> Result<Config, Box<dyn error::Error>> {
    let config_dir = app.path().app_config_dir()?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.toml");
    let config = if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;
        toml::from_str(&contents).unwrap_or_default()
    } else {
        let default_config = Config::default();
        save_config(&app, default_config)?;
        default_config
    };

    Ok(config)
}

pub fn save_config<R: tauri::Runtime, M: Manager<R>>(app: &M, new_config: Config) -> Result<(), Box<dyn error::Error>> {
    // Save config: ALWAYS read from `config` parameter
    // Load: fall back to defaults only when necessary (passing the default struct in here)
    let config_dir = app.path().app_config_dir()?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let toml_string = toml::to_string(&new_config)?;
    fs::write(&config_path, toml_string)?;

    Ok(())
}

// :reload_config
#[tauri::command]
pub fn reload_config(app: tauri::AppHandle, state: tauri::State<ConfigState>) -> Result<(), String> {
    let new_conf = load_config(&app).map_err(|e| format!("Error loading config: {}", e))?;

    // Mutex of config, which is stored in memory (we want to update it)
    let mut config = state.0.lock().unwrap();
    *config = new_conf.clone();

    Ok(())
}

// Config actions from frontend
// add path
// remove path
// edit path
// update ollama (url, default model, alternatives)
#[tauri::command]
pub fn add_edit_path(
    state: tauri::State<ConfigState>,
    app: tauri::AppHandle,
    old_path: String,
    new_path: String,
    default_language_code: String
) -> Result<(), String> {


    if new_path == "" {
        new_path = old_path;
    }

    // TODO: check if default_language-code is actually a valid code

    let mut config = state.0.lock().map_err(|e| e.to_str())?;

    // if still false, error has occured
    let mut has_changed = false;

    if let Some(existing_item) = config.paths.item.iter().find(|i| i.path == old_path && i.path != new_path) {
        existing_item.path = new_path;
        existing_item.default_language = default_language_code;
        has_changed = true;
    } else if let Some(new_item) = config.paths.item.iter().find(|i| i.path != old_path && i.path != new_path) {
        config.paths.item.push(Item {
            path: new_path,
            default_language: default_language_code
        });
        has_changed = true;
    }

    if has_changed {
        Ok(())
    } else {
        Err("Error".into())
    }

}
