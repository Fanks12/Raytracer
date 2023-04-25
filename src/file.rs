use std::fs;

pub fn open_scene(file_path: &String, scene_contents: &mut String) -> bool {
    match fs::read_to_string(file_path) {
        Ok(file_contents) => {
            *scene_contents = file_contents;
            true
        },
        Err(_) => {
            eprintln!("Error: Unable to open scene configuration");
            false
        }
    }
}
