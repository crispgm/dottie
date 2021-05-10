use std::error::Error;

use crate::commands::Command;
use crate::config::Config;

pub struct ListOpt {
    path: String,
}

impl ListOpt {
    pub fn new(path: String) -> ListOpt {
        ListOpt { path }
    }
}

impl Command for ListOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Listing repository => {}", self.path);

        let cfg = Config::from_toml(self.path.clone()).unwrap();
        println!("Name: {}", cfg.name);
        for item in cfg.dotfiles.unwrap().iter() {
            println!("\t- {}", item.name);
        }

        Ok(())
    }
}
