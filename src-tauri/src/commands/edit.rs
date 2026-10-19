use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[tauri::command]
pub fn save_title(title: &str) -> String {
    format!("Title: {}", title)
}


#[tauri::command]
pub fn load_file(filepath: String) -> String{
    // create path
    let path = Path::new(&filepath);
    let display = path.display();

    // open in read-only
    let mut file = match File::open(&path) {
        Err(e) => panic!("Error opening {}: {}", display, e),
        Ok(f) => f,
    };

    // read contents
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!("Couldn't read file {}: {}", display, e),
        Ok(_) => contents,
    }
}
