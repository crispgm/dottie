use std::error::Error;

use crate::commands::Command;

pub struct UnlinkOpt {
    name: String,
}

impl UnlinkOpt {
    pub fn new(name: String) -> UnlinkOpt {
        UnlinkOpt { name }
    }
}

impl Command for UnlinkOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Unlinking => {}", self.name);
        Ok(())
    }
}
