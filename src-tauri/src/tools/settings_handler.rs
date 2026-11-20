use tauri::Manager;
use toml;
use serde::{ Deserialize, Serialize };
use std::fmt::format;
use std::fs;
use std::sync::Mutex;
use std::error;

pub struct ConfigState(pub Mutex<Config>);

enum ConfigDefaults {
    OllamaURL,
    OllamaModel,
    OllamaAlternatives,
    PathItemPath,
    PathItemDefaultLanguage,
}

impl ConfigDefaults {
    // this code is probably horribly written but i js need it to work at the moment..
    fn as_str(&self) -> &str {
        match self {
            ConfigDefaults::OllamaURL => "http://localhost:11434",
            ConfigDefaults::OllamaModel => "gemma4",
            ConfigDefaults::OllamaAlternatives => "",
            ConfigDefaults::PathItemPath => "<default>",
            ConfigDefaults::PathItemDefaultLanguage => "es",
        }
    }

    fn as_int(&self) -> u8 {
        match self {
            ConfigDefaults::OllamaAlternatives => 3,
            _ => 0
        }
    }
}

// Config Structs
#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    ollama: Ollama,
    paths: Paths,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(default)]
struct Ollama {
    url: String,
    default_model: String,
    alternatives: u8,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(default)]
struct Paths {
    item: Vec<Item>,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
#[serde(default)]
struct Item {
    path: String,
    default_language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama: Ollama {
                url: ConfigDefaults::OllamaURL.as_str().into(),
                default_model: ConfigDefaults::OllamaModel.as_str().into(),
                alternatives: ConfigDefaults::OllamaAlternatives.as_int(),
            },
            paths: Paths {
                item: vec!(Item {
                    path: ConfigDefaults::PathItemPath.as_str().into(),
                    default_language: ConfigDefaults::PathItemDefaultLanguage.as_str().into()
                })
            }
        }
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            url: ConfigDefaults::OllamaURL.as_str().into(),
            default_model: ConfigDefaults::OllamaModel.as_str().into(),
            alternatives: ConfigDefaults::OllamaAlternatives.as_int()
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            item: vec!(Item {
                path: ConfigDefaults::PathItemPath.as_str().into(),
                default_language: ConfigDefaults::PathItemDefaultLanguage.as_str().into()
            })
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            path: ConfigDefaults::PathItemPath.as_str().into(),
            default_language: ConfigDefaults::PathItemDefaultLanguage.as_str().into()
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
        toml::from_str(&contents)?
    } else {
        let default_config = Config::default();
        save_config(app, default_config.clone())?;
        default_config
    };

    Ok(config)
}

pub fn save_config<R: tauri::Runtime, M: Manager<R>>(app: &M, mut new_config: Config) -> Result<(), Box<dyn error::Error>> {
    // Save config: ALWAYS read from `config` parameter
    // Load: fall back to defaults only when necessary (passing the default struct in here)
    let config_dir = app.path().app_config_dir()?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.toml");

    // if the default path config is still there and they're adding another
    if new_config.paths.item.len() > 1 && new_config.paths.item.contains(&Item {
        path: ConfigDefaults::PathItemPath.as_str().into(),
        default_language: ConfigDefaults::PathItemDefaultLanguage.as_str().into()
    }) {
        // find the index of the default
        let index = new_config.paths.item.iter().position(|i| i == &Item {
            path: ConfigDefaults::PathItemPath.as_str().into(),
            default_language: ConfigDefaults::PathItemDefaultLanguage.as_str().into()
        }).unwrap();

        new_config.paths.item.remove(index);
    }

    let toml_string = toml::to_string(&new_config)?;
    fs::write(&config_path, toml_string)?;

    Ok(())
}

// :reload_config
#[tauri::command]
pub fn reload_config(app: tauri::AppHandle, state: tauri::State<ConfigState>) -> Result<String, String> {
    let new_conf = load_config(&app).map_err(|e| format!("Error loading config: {}", e))?;

    // Mutex of config, which is stored in memory (we want to update it)
    let mut config = state.0.lock().unwrap();
    *config = new_conf.clone();

    Ok(format!("{:?}", *config))
    //Ok(String::from("Reload Config: OK"))
}

// Config actions from frontend
// add path
// remove path
// edit path
// update ollama (url, default model, alternatives)

#[tauri::command]
pub fn add_path(
    state: tauri::State<ConfigState>,
    app: tauri::AppHandle,
    new_path: String,
    lang_code: String
) -> Result<String, String> {

    let mut config = state.0.lock().map_err(|e| e.to_string())?;

    if lang_code == "" || new_path == "" {
        return Err("Add Path: Lang code or new path is empty".into());
    }

    if !config.paths.item.iter().any(|i| i.path == new_path) {
        config.paths.item.push(Item { path: new_path, default_language: lang_code});
        save_config(&app, config.clone()).map_err(|e| e.to_string())?;
    }

    Ok(String::from("Add Path: OK"))
}

#[tauri::command]
pub fn edit_path(
    state: tauri::State<ConfigState>,
    app: tauri::AppHandle,
    old_path: String,
    new_path: String,
    new_lang_code: String
) -> Result<String, String> {
    Ok(String::new())
}
