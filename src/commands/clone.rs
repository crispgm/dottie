use std::error::Error;

use crate::commands::Command;

pub struct CloneOpt {
    git: String,
}

impl CloneOpt {
    pub fn new(git: String) -> CloneOpt {
        CloneOpt { git }
    }
}

impl Command for CloneOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Cloning repository => {}", self.git);
        Ok(())
    }
}
