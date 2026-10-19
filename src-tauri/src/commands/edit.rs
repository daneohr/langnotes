use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[tauri::command]
pub fn save_title(title: &str) -> String {
    format!("Title: {}", title)
}


#[tauri::command]
pub fn load_file(filepath: String) -> Result<String, String>{
    // create path
    let path = Path::new(&filepath);
    let display = path.display();

    // open in read-only
    let mut file = File::open(&path)
        .map_err(|e| format!("Error opening {}: {}", display, e))?;

    // read contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Couldn't read file {}: {}", display, e))?;

    Ok(contents)
}
