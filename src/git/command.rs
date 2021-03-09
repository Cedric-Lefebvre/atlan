use std::process::Command;

pub fn init()  -> () {
    let mut list_dir = Command::new("git")
        .arg("init")
        .arg("/tmp/test")
        .status()
        .expect("process failed to execute");
}

pub fn add()  -> () {
    let mut list_dir = Command::new("git")
        .arg("-C")
        .arg("/tmp/test")
        .arg("add")
        .arg(".")
        .status()
        .expect("process failed to execute");
}

pub fn commit()  -> () {
    let mut list_dir = Command::new("git")
        .arg("-C")
        .arg("/tmp/test")
        .arg("commit")
        .arg("-m")
        .arg("test")
        .status()
        .expect("process failed to execute");
}

pub fn push()  -> () {
    let mut list_dir = Command::new("git")
        .arg("-C")
        .arg("/tmp/test")
        .arg("push")
        .status()
        .expect("process failed to execute");
}