use std::error::Error;

use crate::commands::Command;
use crate::show_info;

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
        show_info!("Unlinking => {}", self.name);
        Ok(())
    }
}
