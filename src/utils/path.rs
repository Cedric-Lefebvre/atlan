pub fn get_home_path() -> std::string::String {
    match dirs::home_dir() {
        Some(path) => return path.display().to_string(),
        None => return "Impossible to get your home dir!".to_string(),
    };
}