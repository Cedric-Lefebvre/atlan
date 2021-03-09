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


pub fn get_args() -> () {
    match Cli::from_args() {
        Cli::Config { path } => {
            println!("{:?}", path)
        },
        Cli::Backup { path } => {
            println!("test")
        }
        _ => (),
    }
}