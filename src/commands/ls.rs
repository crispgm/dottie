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
        if cfg.description.is_some() {
            println!("Name: {} ({})", cfg.name, cfg.description.unwrap());
        } else {
            println!("Name: {}", cfg.name);
        }
        for item in cfg.dotfiles.unwrap().iter() {
            println!("\t- {}", item.name);
        }

        Ok(())
    }
}

#[test]
fn test_ls() {
    let path = String::from("./fixtures/dottie.toml");
    let opt: ListOpt = ListOpt::new(path.clone());
    assert_eq!(opt.path, path)
}
