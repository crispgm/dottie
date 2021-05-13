use std::error::Error;
use std::process;

use crate::commands::Command;
use crate::config::Config;
use crate::{show_error, show_info};

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
        show_info!("Listing repository => {}", self.path);

        let cfg = Config::from_toml(self.path.clone()).unwrap_or_else(|err| {
            show_error!("{}", err);
            process::exit(1);
        });

        show_info!("{}", cfg.brief());
        for item in cfg.dotfiles.unwrap().iter() {
            show_info!("\t- {}", item.name);
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
