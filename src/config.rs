use serde_yaml;
use serde;
use serde::{Deserialize, Serialize}; 
use std::fs::File;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
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

pub fn get_config() -> Data {
    let f = File::open("src/something.yaml").expect("File not found");
    serde_yaml::from_reader(f).unwrap()
}


use std::io::prelude::*;

pub fn create_config()  -> () {
    let content = r#"git_repo: https://github.com/user/repo.git
environment:
    GITHUB_USER: #If you want to override user's default
    GITHUB_TOKEN: #If you want to override user's default
files:
    - #filePath
"#;

    let mut file = File::create("config.yaml").expect("Error occured when creating the file");
    file.write_all(content.as_bytes()).expect("Error occured when writing to file");
}










// #[derive(Debug, serde::Deserialize, PartialEq)]
// struct Build {
//     build: Vec<Foo>,
// }

// #[derive(Debug, serde::Deserialize, PartialEq)]
// #[serde(untagged)]
// enum Foo {
//     Step(Step),
//     Bar(String),
// }

// #[derive(Debug, serde::Deserialize, PartialEq)]
// struct Step {
//     name: String,
//     #[serde(rename = "do")]
//     make: Option<String>,
//     put: Option<String>,
//     get: Option<String>,
// }