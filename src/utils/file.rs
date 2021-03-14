use std::path::Path;
use crate::cli::arguments::get_home;

#[derive(Debug)]
pub struct File {
    pub file_name: String,
    pub folder_path: String,
    pub tmp_folder_path: String,
    pub tmp_file_path: String,
    pub real_folder_path: String,
    pub real_file_path: String
}

impl File {
    pub fn new(path: String, backup_path: String) -> Self {
        let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap().to_string();   // settings.json
        let folder_path = path.replace(&file_name, "");   // ~/.config/Code/User/
        let tmp_folder_path = format!("{}/{}", backup_path, folder_path); // /tmp/{timestamp}/~/.config/Code/User/
        let tmp_file_path = format!("{}{}", tmp_folder_path, file_name); // /tmp/{timestamp}/~/.config/Code/User/settings.json
        let mut real_folder_path = folder_path.to_string(); // /home/{username}/.config/Code/User/
        if folder_path.starts_with("~") {
            real_folder_path.remove(0);
            real_folder_path = get_home() + &real_folder_path
        }
        let real_file_path = real_folder_path.clone() + &file_name; // /home/{username}/.config/Code/User/settings.json

        Self {
            file_name,
            folder_path,
            tmp_folder_path,
            tmp_file_path,
            real_folder_path,
            real_file_path
        }
    }
}