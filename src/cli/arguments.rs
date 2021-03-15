use structopt::StructOpt;

#[derive(StructOpt)]
#[derive(Debug)]
#[structopt(about = "Backup/Restore tool")]
pub enum Cli {
    #[structopt(about = "Configuration file")]
    Config {
        #[structopt(long = "--create", help = "Path")]
        create: bool,
        #[structopt(long = "--delete", help = "Path")]
        delete: bool
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