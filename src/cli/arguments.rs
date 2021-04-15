use structopt::StructOpt;

#[derive(StructOpt)]
#[derive(Debug)]
#[structopt(about = "Backup/Restore tool")]
pub enum Cli {
    #[structopt(about = "Configuration file")]
    Config(ConfigOpts),
    #[structopt(about = "Backup configuration from local computer")]
    Backup {},
    #[structopt(about = "Restore configuration on local computer")]
    Restore {}
}

#[derive(StructOpt,Debug)]
pub enum ConfigOpts {
    Create {},
    Delete {},
    View {}
}