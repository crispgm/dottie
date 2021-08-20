use std::error::Error;

use crate::commands::Command;
use crate::show_info;

pub struct CloneOpt {
    git: String,
}

impl CloneOpt {
    pub fn new(git: &str) -> CloneOpt {
        CloneOpt {
            git: git.to_string(),
        }
    }
}

impl Command for CloneOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        show_info!("Cloning repository => {}", self.git);
        Ok(())
    }
}
