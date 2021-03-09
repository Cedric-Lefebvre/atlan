use std::process::Command;

pub fn init(path: String) -> () {
    Command::new("git")
        .arg("init")
        .arg(path)
        .status()
        .expect("process failed to execute");
}

pub fn remote(path: String, remote: String) -> () {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(remote)
        .status()
        .expect("process failed to execute");
}

pub fn pull(path: String, remote: String) -> () {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("pull")
        .arg(remote)
        .arg("master")
        .status()
        .expect("process failed to execute");
}

pub fn add(path: String) -> () {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("add")
        .arg(".")
        .status()
        .expect("process failed to execute");
}

pub fn commit(path: String, message: String) -> () {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .expect("process failed to execute");
}

pub fn push(path: String) -> () {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("push")
        .arg("--set-upstream")
        .arg("origin")
        .arg("master")
        .status()
        .expect("process failed to execute");
}