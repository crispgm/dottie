use std::error::Error;
use std::fs::metadata;
use std::path::PathBuf;

use crate::commands::Command;
use crate::config::{DotItem, DotType};
use crate::show_info;

pub struct AddOpt {
    name: String,
    path: String,
}

impl AddOpt {
    pub fn new(name: String, path: String) -> AddOpt {
        AddOpt { name, path }
    }

    fn is_dir(&self) -> bool {
        let md = metadata(self.path.clone()).unwrap();
        return md.is_dir();
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
        let is_dir = self.is_dir();
        if is_dir {
            show_info!("Adding directory => {}", self.path);
            item.dot_type = DotType::Dir;
        } else {
            show_info!("Adding file => {}", self.path);
        }
        println!("{:?}", item);
        // TODO: check whether target is dottied
        // TODO: do adding
        Ok(())
    }
}
