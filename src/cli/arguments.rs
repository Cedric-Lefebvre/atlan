use structopt::StructOpt;

#[derive(StructOpt)]
#[derive(Debug)]
#[structopt(about = "Backup/Restore tool")]
pub enum Cli {
    #[structopt(about = "Configuration file")]
    Config {
        #[structopt(short = "e", long = "--path", help = "Path", default_value = "config.yaml")]
        path: String,
    },
    #[structopt(about = "Backup configuration from local computer")]
    Backup {
        #[structopt(short = "e", long = "--path", help = "Path", default_value = "config.yaml")]
        path: String,
    },
    #[structopt(about = "Restore configuration on local computer")]
    Restore {
        #[structopt(short = "e", long = "--path", help = "Path", default_value = "config.yaml")]
        path: String,
    }
}


use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::os::unix::prelude::*;
use crate::settings;
use std::path::Path;
use crate::git;

pub fn get_args() -> () {
    match Cli::from_args() {
        Cli::Config { path } => {
            println!("{:?}", path)
        },
        Cli::Backup { path } => {
            let duration =  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error occurred");
            let time = duration.as_secs() * 1000 +
                duration.subsec_nanos() as u64 / 1_000_000;
            let backup_path: String = format!("/tmp/{}", time);
            let config = settings::config::get_config();    

            git::command::init(backup_path.clone());
            git::command::remote(backup_path.clone(), config.git_repo.clone());
            git::command::pull(backup_path.clone(), config.git_repo.clone());

            for origin in config.files {
                let file_name = Path::new(&origin).file_name().unwrap().to_str().unwrap();   // settings.json
                let file_path = &origin.replace(file_name, "");   // ~/.config/Code/User/
                let full_folder_path = format!("{}/{}", backup_path, file_path); // /tmp/1615323956137/~/.config/Code/User/
                let full_file_path = format!("{}{}", full_folder_path, file_name);   // /tmp/1615323956137/~/.config/Code/User/settings.json
                
                let mut resolved_path: String = file_path.to_string(); // /home/username/.config/Code/User/
                if file_path.starts_with("~") {
                    resolved_path.remove(0);
                    resolved_path = get_home() + &resolved_path
                }
                resolved_path += file_name; // /home/username/.config/Code/User/settings.json

                fs::create_dir_all(full_folder_path).expect("Error when creating folder");
                fs::copy(resolved_path, full_file_path).expect("Error when copying file");
            }

            git::command::add(backup_path.clone());
            git::command::commit(backup_path.clone(), time.to_string());
            git::command::push(backup_path.clone());

            fs::remove_dir_all(backup_path.clone()).expect("Error when deleting folder");
            println!("{:?}", time);
        }
        _ => (),
    }
}

fn get_home() -> std::string::String {
    match dirs::home_dir() {
        Some(path) => return path.display().to_string(),
        None => return "Impossible to get your home dir!".to_string(),
    };
}