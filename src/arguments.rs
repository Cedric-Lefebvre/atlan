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
// use crate::config;

pub fn get_args() -> () {
    match Cli::from_args() {
        // Cli::Config { path } => {
        //     println!("{:?}", path)
        // },
        Cli::Backup { path } => {
            let duration =  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error occurred");
            let time = duration.as_secs() * 1000 +
                duration.subsec_nanos() as u64 / 1_000_000;
            let path = format!("/tmp/{}", time);
            fs::create_dir(path).expect("Error when creating folder");

            println!("{:?}", time)

            // let test = config::get_config();

            // println!("{:#?}", test);
        }
        _ => (),
    }
}