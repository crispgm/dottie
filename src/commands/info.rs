use std::error::Error;

use crate::commands::Command;
use crate::config::Config;

pub struct InfoOpt {
    path: String,
    name: String,
}

impl InfoOpt {
    pub fn new(path: String, name: String) -> InfoOpt {
        InfoOpt { path, name }
    }
}

impl Command for InfoOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Show info => {}", self.name);
        let cfg = Config::from_toml(self.path.clone()).unwrap();
        let item = cfg.get_by_name(self.name.to_string());
        match item {
            Some(di) => {
                if di.symlinked.is_some() && di.symlinked.unwrap() {
                    println!("{} ✅", di.name);
                } else {
                    println!("{}", di.name);
                }
                println!("Source: {}", di.src);
                println!("Target: {}", di.target);
            }
            None => eprintln!("Dotfile `{}` not found", self.name),
        }
        Ok(())
    }
}
