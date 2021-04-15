mod cli;
mod git;
mod settings;
mod utils;

use crate::cli::arguments::{Cli, ConfigOpts};
use crate::git::command::Git;
use crate::utils::file::File;
use crate::utils::time::get_timestamp;
use ansi_term::Colour;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

pub fn main() {
    match Cli::from_args() {
        Cli::Config(action) => match action {
            ConfigOpts::Create {} => return settings::config::create_config(),
            ConfigOpts::Delete {} => return settings::config::delete_config(),
            ConfigOpts::View {} => return settings::config::get_config_content(),
        },
        Cli::Backup {} => {
            let time = get_timestamp();
            let backup_path: String = format!("/tmp/{}", time);
            let config = settings::config::get_config();
            let git = Git {
                path: backup_path.clone(),
                repo: config.git_repo.clone(),
            };
            git.init();
            git.remote();
            git.pull();

            for origin in config.files {
                let file = File::new(origin.clone(), backup_path.clone());
                print!("{} \n", Colour::Yellow.paint(origin));
                fs::create_dir_all(file.tmp_folder_path).expect("Error when creating folder");
                fs::copy(file.real_file_path, file.tmp_file_path).expect("Error when copying file");
            }

            git.add();
            git.commit(time.to_string());
            git.push();

            fs::remove_dir_all(backup_path).expect("Error when deleting folder");
            print!("{}", Colour::Green.paint("Backup completed \n"));
        }
        Cli::Restore {} => {
            let time = get_timestamp();
            let backup_path: String = format!("/tmp/{}", time);
            let config = settings::config::get_config();
            let git = Git {
                path: backup_path.clone(),
                repo: config.git_repo.clone(),
            };
            git.init();
            git.remote();
            git.pull();

            for destination in config.files {
                let file = File::new(destination.clone(), backup_path.clone());
                print!("{} \n", Colour::Yellow.paint(destination));
                if !Path::new(&file.real_folder_path).exists() {
                    fs::create_dir_all(&file.real_folder_path).expect("Error when creating folder");
                }
                fs::copy(file.tmp_file_path, &file.real_file_path)
                    .expect("Error when copying file");
            }
            print!("{}", Colour::Green.paint("Restauration completed \n"));
            fs::remove_dir_all(backup_path).expect("Error when deleting folder");
        }
    }
}
