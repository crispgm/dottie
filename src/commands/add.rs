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
    pub fn new(path: &str, name: &str, src: &str) -> AddOpt {
        AddOpt {
            name: name.to_string(),
            path: path.to_string(),
            src: src.to_string(),
        }
    }

    fn default_name(&self, src: &PathBuf) -> String {
        if self.name.is_empty() {
            // if name is not set, convert fn.ext to fn_ext
            let name = src.file_name().unwrap().to_str().unwrap().replace(".", "_");
            return name;
        }

        self.name.to_string()
    }
}

impl Command for AddOpt {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut cfg = Config::from_toml(&PathBuf::from(&self.path)).unwrap();
        let src = PathBuf::from(&self.src);
        if cfg.is_dottied(&src) {
            show_error!("Source dotfile `{}` has already been dottied", self.src);
            return Err(Box::new(SourceFileIsDottied));
        }
        let mut item = DotItem {
            name: self.default_name(&src).to_string(),
            src,
            target: PathBuf::from(&self.path), // TODO: expand source and target
            dot_type: DotType::File,
            symlinked: false,
        };
        // TODO: check whether target is dottied
        let f = File::from_path(&item.src);
        if f.is_symlink() {
            show_error!(
                "Source dotfile `{}` has been already a symbolic link",
                self.src
            );
            return Err(Box::new(SymlinkSourceNotSupported));
        } else if f.is_dir() {
            show_info!("Adding directory => {}", self.src);
            item.dot_type = DotType::Dir;
        } else if f.is_file() {
            show_info!("Adding file => {}", self.src);
        } else {
            // TODO: unknown
            show_error!("Unknown file type `{}`", self.src);
        }

        match cfg.add(item) {
            Err(e) => return Err(e),
            _ => (),
        }
        cfg.save()
        // TODO: move the files
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
        let add = AddOpt::new(".", "", "./init.vim");
        let def_name = add.default_name(&PathBuf::from("init.vim"));
        assert_eq!("init_vim", def_name);
    }
}
