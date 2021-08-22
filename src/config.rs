use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DotType {
    File,
    Dir,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DotItem {
    pub name: String,
    #[serde(rename = "dt")]
    pub dot_type: DotType,
    pub src: PathBuf,
    pub target: PathBuf,
    pub symlinked: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub repo: String,
    pub description: String,
    pub dotfiles: Vec<DotItem>,
}

#[derive(Debug, Clone)]
pub struct ConfigNotExisted;

impl fmt::Display for ConfigNotExisted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config is not existed")
    }
}

impl Error for ConfigNotExisted {}

impl Config {
    // load from `dottie.toml`
    pub fn from_toml(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
        if !Path::new(&path).exists() {
            return Err(Box::new(ConfigNotExisted));
        }

        let fs = read_to_string(&path).unwrap();
        let cfg: Config = toml::from_str(fs.as_str()).unwrap();
        Ok(cfg)
    }

    pub fn brief(&self) -> String {
        if self.description.is_empty() {
            format!("Name: {}", self.name)
        } else {
            format!("Name: {} ({})", self.name, self.description,)
        }
    }

    pub fn get_by_name(&self, name: String) -> Option<DotItem> {
        for item in self.dotfiles.iter() {
            if item.name == name {
                return Some(item.clone());
            }
        }
        None
    }

    pub fn is_dottied(&self, src: &PathBuf) -> bool {
        for item in self.dotfiles.iter() {
            if item.src.eq(src) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, item: DotItem) -> Result<(), Box<dyn Error>> {
        self.dotfiles.push(item);

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::config::*;
    use std::path::PathBuf;

    #[test]
    fn load_toml_not_exists() {
        let e = Config::from_toml(&PathBuf::from("/some/path/not/exists"))
            .unwrap_err()
            .to_string();
        assert_eq!(e.starts_with("Config is not existed"), true)
    }

    #[test]
    fn load_from_toml() {
        let result = Config::from_toml(&PathBuf::from("./fixtures/dottie.toml")).unwrap();
        assert_eq!(result.brief(), "Name: dottie_example (Example dottie.toml)");
        let item = result.get_by_name("nvim".to_string()).unwrap();
        assert_eq!(item.name, "nvim");
        assert_eq!(result.name, "dottie_example");
        assert_eq!(result.repo, "git@github.com:crispgm/dottie.git");
        assert_eq!(result.is_dottied(&PathBuf::from("./nvim")), true)
    }

    #[test]
    fn add_to_conf() {
        let mut dt = Config {
            name: "test".to_string(),
            repo: "".to_string(),
            description: "".to_string(),
            dotfiles: vec![],
        };
        let rs = dt
            .add(DotItem {
                name: "_test".to_string(),
                dot_type: DotType::File,
                src: PathBuf::from("abc"),
                target: PathBuf::from(""),
                symlinked: false,
            })
            .unwrap();
        assert_eq!(rs, ());
        assert_eq!(dt.is_dottied(&PathBuf::from("abc")), true);
    }
}
