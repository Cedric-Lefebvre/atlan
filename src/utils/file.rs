use std::path::Path;
use crate::cli::arguments::get_home;

#[derive(Debug)]
pub struct File {
    pub file_name: String,
    pub file_path: String,
    pub full_folder_path: String,
    pub full_file_path: String,
    pub real_path: String
}

impl File {
    pub fn new(path: String, backup_path: String) -> Self {
        
        let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap().to_string();   // settings.json
        let file_path = path.replace(&file_name, "");   // ~/.config/Code/User/
        let full_folder_path = format!("{}/{}", backup_path, file_path); // /tmp/{timestamp}/~/.config/Code/User/
        let full_file_path = format!("{}{}", full_folder_path, file_name); // /tmp/{timestamp}/~/.config/Code/User/settings.json
        let mut real_path = file_path.to_string(); // /home/{username}/.config/Code/User/
        if file_path.starts_with("~") {
            real_path.remove(0);
            real_path = get_home() + &real_path
        }

        Self {
            file_name,
            file_path,
            full_folder_path,
            full_file_path,
            real_path
        }
    }
}