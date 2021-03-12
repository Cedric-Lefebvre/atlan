#[allow(warnings)]  //Disable warnings temporarly

mod cli;
mod settings;
mod git;

fn main() {
    let args = cli::arguments::get_args();
    // println!("{:?}", args);


    // let config = settings::config::get_config();

    // println!("{:#?}", config);
    // config::create_config();
}