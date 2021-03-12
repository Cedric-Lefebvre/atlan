use std::process::Command;
use std::fmt::Debug;

#[derive(Debug)] 
pub struct Git {
    pub path: String,
    pub repo: String
}

impl Git {
    pub fn init(&self) -> () {
        println!("{:?}",self);
        Command::new("git")
            .arg("init")
            .arg(&self.path)
            .status()
            .expect("process failed to execute");
    }
    
    pub fn remote(&self) -> () {
        Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&self.repo)
            .status()
            .expect("process failed to execute");
    }
    
    pub fn pull(&self) -> () {
        Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("pull")
            .arg(&self.repo)
            .arg("master")
            .status()
            .expect("process failed to execute");
    }
    
    pub fn add(&self) -> () {
        Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("add")
            .arg(".")
            .status()
            .expect("process failed to execute");
    }
    
    pub fn commit(&self, message: String) -> () {
        Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("commit")
            .arg("-m")
            .arg(message)
            .status()
            .expect("process failed to execute");
    }
    
    pub fn push(&self) -> () {
        Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg("master")
            .status()
            .expect("process failed to execute");
    }
}
