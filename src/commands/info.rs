use std::error::Error;

use crate::commands::Command;
use crate::config::Config;
use crate::{show_error, show_info};

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
        show_info!("Show info => {}", self.name);
        let cfg = Config::from_toml(self.path.clone()).unwrap();
        let item = cfg.get_by_name(self.name.to_string());
        match item {
            Some(di) => {
                if di.symlinked.is_some() && di.symlinked.unwrap() {
                    show_info!("{} ✅", di.name);
                } else {
                    show_info!("{}", di.name);
                }
                show_info!("Source: {}", di.src.into_os_string().into_string().unwrap());
                show_info!(
                    "Target: {}",
                    di.target.into_os_string().into_string().unwrap()
                );
            }
            None => show_error!("Dottie file `{}` not found", self.name),
        }
        Ok(())
    }
}
