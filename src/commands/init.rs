use std::error::Error;

use crate::command::exec;
use crate::commands::Command;

pub struct InitOpt {
    git: String,
}

// init repo with git (optional)
impl InitOpt {
    pub fn new(git: String) -> InitOpt {
        InitOpt { git }
    }
}

impl Command for InitOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let git = self.git.trim();
        println!("Initializing with git repository => {}", git);
        if git.len() > 0 {
            let args = vec![String::from("remote"), String::from("-v")];
            let output = exec(String::from("git"), args).unwrap();
            let lines = String::from_utf8(output.stdout)?;
            println!("Output: {}", lines);
        }
        Ok(())
    }
}
