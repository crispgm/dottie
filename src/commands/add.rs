use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::commands::Command;
use crate::config::Config;
use crate::config::{DotItem, DotType};
use crate::fs::File;
use crate::{show_error, show_info};

pub struct AddOpt {
    path: String,
    name: String,
    src: String,
}

impl AddOpt {
    pub fn new(path: String, name: String, src: String) -> AddOpt {
        AddOpt { name, path, src }
    }

    fn default_name(&self, src: PathBuf) -> String {
        let mut name = self.name.clone();
        if self.name.is_empty() {
            // if name is not set, convert fn.ext to fn_ext
            name = src.file_name().unwrap().to_str().unwrap().replace(".", "_");
        }

        name
    }
}

impl Command for AddOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let cfg = Config::from_toml(self.path.clone()).unwrap();
        let src = PathBuf::from(self.src.clone());
        if cfg.is_dottied(src.clone()) {
            show_error!("Source dotfile `{}` has already been dottied", self.path);
            return Err(Box::new(SourceFileIsDottied));
        }
        let mut item = DotItem {
            name: self.default_name(src.clone()),
            src,
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
            );
            return Err(Box::new(SymlinkSourceNotSupported));
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

#[derive(Debug, Clone)]
pub struct SymlinkSourceNotSupported;

impl fmt::Display for SymlinkSourceNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source file cannot be a symbolic link")
    }
}

impl Error for SymlinkSourceNotSupported {}

#[derive(Debug, Clone)]
pub struct SourceFileIsDottied;

impl fmt::Display for SourceFileIsDottied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source file has already been dottied")
    }
}

impl Error for SourceFileIsDottied {}

#[cfg(test)]
mod test {
    use crate::commands::add::*;

    #[test]
    fn convert_to_default_name() {
        let add = AddOpt::new(".".to_string(), "".to_string(), "./init.vim".to_string());
        let def_name = add.default_name(PathBuf::from("init.vim"));
        assert_eq!("init_vim", def_name);
    }
}
