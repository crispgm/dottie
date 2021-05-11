use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;

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
    pub src: String,
    pub target: String,

    pub os: Option<String>, // respect std::env::consts::OS

    pub symlinked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub repo: Option<String>,
    pub description: Option<String>,
    pub dotfiles: Option<Vec<DotItem>>,
}

/// ConfigNotExisted error for loading toml file
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
    // TODO: support custom file path
    pub fn from_toml(path: String) -> Result<Config, Box<dyn Error>> {
        if !Path::new(&path).exists() {
            eprintln!("ConfigNotExisted error: `{}` not exists", path);
            return Err(Box::new(ConfigNotExisted));
        }

        let fs = read_to_string(&path).unwrap();
        let cfg: Config = toml::from_str(fs.as_str()).unwrap();
        Ok(cfg)
    }

    // filter with things like os, etc.
}

#[test]
fn load_toml_not_exists() {
    let e = Config::from_toml(String::from("/some/path/not/exists"))
        .unwrap_err()
        .to_string();
    assert_eq!(e.starts_with("Config is not existed"), true)
}

#[test]
fn load_from_toml() {
    let result = Config::from_toml(String::from("./fixtures/dottie.toml")).unwrap();
    assert_eq!(result.name, "dottie_example");
    assert_eq!(result.repo.unwrap(), "git@github.com:crispgm/dottie.git");
}
