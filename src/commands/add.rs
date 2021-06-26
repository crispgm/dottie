use std::error::Error;
use std::path::PathBuf;

use crate::commands::Command;
use crate::config::{DotItem, DotType};
use crate::fs::File;
use crate::{show_error, show_info};

pub struct AddOpt {
    name: String,
    path: String,
}

impl AddOpt {
    pub fn new(name: String, path: String) -> AddOpt {
        AddOpt { name, path }
    }
}

impl Command for AddOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: if name is "", then what to use?
        let mut item = DotItem {
            name: self.name.clone(),
            src: PathBuf::from(self.path.clone()),
            target: PathBuf::from(self.path.clone()), // TODO: expand source and target
            dot_type: DotType::File,
            symlinked: None,
        };
        // TODO: check whether target is dottied
        let f = File::from_path(item.src.clone());
        if f.is_symlink() {
            show_error!(
                "Source dotfile `{}` has been already a symbolic link",
                self.path
            )
            // TODO: return the error
        } else if f.is_dir() {
            show_info!("Adding directory => {}", self.path);
            item.dot_type = DotType::Dir;
        } else if f.is_file() {
            show_info!("Adding file => {}", self.path);
        } else {
            // TODO: unknown
        }
        println!("{:?}", item);
        // TODO: do adding
        Ok(())
    }
}
