use ansi_term::Colour;
use serde_yaml;
use serde;
use serde::{Deserialize, Serialize}; 
use std::collections::BTreeMap;
use std::fs::{File,create_dir_all,remove_file,read_to_string};
use std::io::prelude::*;
use std::path::Path;
use crate::settings::constant::CONFIG_CONTENT;
use crate::utils::path::get_home_path;
use crate::settings::constant::CONFIG_PATH;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub git_repo: String,
    pub files: Vec<String>,
    pub environment: Environment,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Environment {
    Vec(Vec<String>),
    Hash(BTreeMap<String, String>),
}

pub fn get_config() -> Config {
    let location: String = format!("{}/{}", get_home_path(), CONFIG_PATH);
    let path = std::path::Path::new(&location);

    let f = File::open("config.yaml").expect("File not found");
    serde_yaml::from_reader(f).unwrap()
}

pub fn get_config_content() -> () {
    let location: String = format!("{}/{}", get_home_path(), CONFIG_PATH);
    let path = std::path::Path::new(&location);

    println!("{}", read_to_string(path).expect("error reading file"));
}

pub fn create_config() -> () {
    let location: String = format!("{}/{}", get_home_path(), CONFIG_PATH);
    let path = std::path::Path::new(&location);
    
    if !Path::new(path.parent().unwrap()).exists() {
        create_dir_all(&path.parent().unwrap()).expect("Error when creating folder");
    }
    let mut file = File::create(path).expect("Error occured when creating the file");
    file.write_all(CONFIG_CONTENT.as_bytes()).expect("Error occured when writing to file");
    
    return println!("{} {}",
        Colour::Green.paint("Config file created:"),
        Colour::Yellow.paint("/home/cedric/.config/atlan/config.yaml")
    )
}

pub fn delete_config() -> () {
    let location: String = format!("{}/{}", get_home_path(), CONFIG_PATH);
    let path = std::path::Path::new(&location);

    remove_file(path).expect("Error occured when deleeting the file");
    return println!("{}", Colour::Green.paint("Config file deleted \n"))
}