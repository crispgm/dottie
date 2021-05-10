use std::error::Error;

use crate::commands::Command;

pub struct AddOpt {
    name: String,
}

impl AddOpt {
    pub fn new(name: String) -> AddOpt {
        AddOpt { name }
    }
}

impl Command for AddOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Adding directory => {}", self.name);
        Ok(())
    }
}
