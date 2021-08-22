use std::error::Error;
use std::path::PathBuf;

use crate::commands::Command;
use crate::config::Config;
use crate::{show_error, show_info};

pub struct InfoOpt {
    path: String,
    name: String,
}

impl InfoOpt {
    pub fn new(path: &str, name: &str) -> InfoOpt {
        InfoOpt {
            path: path.to_string(),
            name: name.to_string(),
        }
    }
}

impl Command for InfoOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        show_info!("Show info => {}", self.name);
        let cfg = Config::from_toml(&PathBuf::from(&self.path)).unwrap();
        let item = cfg.get_by_name(self.name.to_string());
        match item {
            Some(di) => {
                if di.symlinked {
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
