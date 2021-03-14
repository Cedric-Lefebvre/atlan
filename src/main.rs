mod cli;
mod settings;
mod git;
mod utils;

use std::fs;
use std::path::Path;
use crate::git::command::Git;
use crate::cli::arguments::Cli;
use structopt::StructOpt;
use crate::utils::time::get_timestamp;
use crate::utils::file::File;

pub fn main() {
    match Cli::from_args() {
        Cli::Config { path } => {
            println!("{:?}", path)
        },
        Cli::Backup { path } => {
            let time = get_timestamp();
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
                let file = File::new(origin, backup_path.clone());
                fs::create_dir_all(file.tmp_folder_path).expect("Error when creating folder");
                fs::copy(file.real_file_path, file.tmp_file_path).expect("Error when copying file");
            }

            git.add();
            git.commit(time.to_string());
            git.push();

            fs::remove_dir_all(backup_path).expect("Error when deleting folder");
            println!("{:?}", time);
        },
        Cli::Restore { path } => {
            let time = get_timestamp();
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
                let file = File::new(destination, backup_path.clone());
                if !Path::new(&file.real_folder_path).exists() {
                    fs::create_dir_all(&file.real_folder_path).expect("Error when creating folder");
                }

                fs::copy(file.tmp_file_path, &file.real_file_path).expect("Error when copying file");
            }

            fs::remove_dir_all(backup_path).expect("Error when deleting folder");
        }
    }
}