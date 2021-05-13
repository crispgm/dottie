use std::error::Error;

use crate::commands::Command;
use crate::show_info;

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
        show_info!("Adding directory => {}", self.name);
        Ok(())
    }
}
