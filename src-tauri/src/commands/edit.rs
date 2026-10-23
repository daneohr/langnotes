use std::fs::{File, write};
use std::io::prelude::*;
use std::path::Path;

#[tauri::command]
pub fn save_title(title: &str) -> String {
    format!("Title: {}", title)
}


#[tauri::command]
pub fn load_file(filepath: String) -> Result<String, String>{
    // TODO: Check filetypes

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

    //TODO: LOAD EDIT PAGE WITH FILE CONTENTS
    Ok(contents)
}

#[tauri::command]
pub fn save_file(filepath: String, contents: String) -> Result<String, String> {
    //TODO: Read Result directly as message in front end

    // std::fs::write
    let path = Path::new(&filepath);
    let display = path.display();

    print!("Path: {}\n", display);
    print!("Contents: {}", contents);
    
    match write(path, contents) {
        Ok(_) => Ok(format!("Saved file: {}", display)),
        Err(e) => Err(format!("Error saving file {}: {}", display, e))
    }
}
