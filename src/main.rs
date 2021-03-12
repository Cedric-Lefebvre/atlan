#[allow(warnings)]  //Disable warnings temporarly

mod cli;
mod settings;
mod git;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use crate::git::command::Git;
use crate::cli::arguments::{Cli,get_home};
use structopt::StructOpt;

pub fn main() {
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
            
            let git = Git {
                path: backup_path.clone(),
                repo: config.git_repo.clone()
            };
            
            git.init();
            git.remote();
            git.pull();

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

            git.add();
            git.commit(time.to_string());
            // git.push();

            fs::remove_dir_all(backup_path.clone()).expect("Error when deleting folder");
            println!("{:?}", time);
        },
        Cli::Restore { path } => {
            let duration =  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error occurred");
            let time = duration.as_secs() * 1000 +
                duration.subsec_nanos() as u64 / 1_000_000;
            let backup_path: String = format!("/tmp/{}", time);
            let config = settings::config::get_config();
            
            let git = Git {
                path: backup_path.clone(),
                repo: config.git_repo.clone()
            };
            
            git.init();
            git.remote();
            git.pull();

            for destination in config.files {
                let file_name = Path::new(&destination).file_name().unwrap().to_str().unwrap();   // settings.json
                let file_path = &destination.replace(file_name, "");   // ~/.config/Code/User/
                let full_folder_path = format!("{}/{}", backup_path, file_path); // /tmp/1615323956137/~/.config/Code/User/
                let full_file_path = format!("{}{}", full_folder_path, file_name);   // /tmp/1615323956137/~/.config/Code/User/settings.json
                
                let mut resolved_path: String = file_path.to_string(); // /home/username/.config/Code/User/
                if file_path.starts_with("~") {
                    resolved_path.remove(0);
                    resolved_path = get_home() + &resolved_path
                }
                resolved_path += file_name; // /home/username/.config/Code/User/settings.json
                
                if !Path::new(&resolved_path).exists() {
                    fs::create_dir_all(resolved_path.clone()).expect("Error when creating folder");
                }

                fs::copy(full_file_path, resolved_path.clone()).expect("Error when copying file");
            }

            fs::remove_dir_all(backup_path.clone()).expect("Error when deleting folder");
        }
    }
}