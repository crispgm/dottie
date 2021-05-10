use std::error::Error;

use crate::commands::Command;

pub struct CDOpt {
    name: String,
}

impl CDOpt {
    pub fn new(name: String) -> CDOpt {
        CDOpt { name }
    }
}

impl Command for CDOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Changing directory => {}", self.name);
        Ok(())
    }
}
