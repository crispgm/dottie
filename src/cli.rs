extern crate clap;

use std::env;

use clap::{App, Arg};

use crate::commands::*;

pub fn run() {
    let app = init_app();

    let matches = app.get_matches();
    // ls
    if let Some(ref _matches) = matches.subcommand_matches("ls") {
        let cwd = get_cwd();
        let path = format!("{}/dottie.toml", cwd);
        if let Err(e) = ls::ListOpt::new(path).run() {
            println!("Running command on {} failed: {}", "ls", e)
        }
    }
    // info
    if let Some(ref matches) = matches.subcommand_matches("info") {
        let cwd = get_cwd();
        let path = format!("{}/fixtures/dottie.toml", cwd);
        let name = matches.value_of("NAME").unwrap_or("");
        if let Err(e) = info::InfoOpt::new(path, name.to_string()).run() {
            println!("Running command on {} failed: {}", "info", e)
        }
    }
    // init
    if let Some(ref matches) = matches.subcommand_matches("init") {
        let git_repo = matches.value_of("git").unwrap_or("");
        let init_opt = init::InitOpt::new(git_repo.to_string());
        if let Err(e) = init_opt.run() {
            println!("Running command on {} failed: {}", "init", e)
        }
    }
}

fn init_app() -> App<'static> {
    App::new("dottie")
        .version("1.0.0")
        .author("David Zhang")
        .arg(
            Arg::new("v")
                .about("Sets the level of verbosity")
                .short('v')
                .multiple(true)
                .takes_value(true),
        )
        .subcommand(
            App::new("clone")
                .about("Init the current folder as a dottie repository")
                .arg(
                    Arg::new("REPOSITORY")
                        .about("Clone a dottie git repository")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("init")
                .about("Init the current folder as a dottie repository")
                .arg(
                    Arg::new("git")
                        .about("Set git repository")
                        .long("--git")
                        .takes_value(true),
                ),
        )
        .subcommand(App::new("ls").about("List each item of dotfiles in current repository"))
        .subcommand(
            App::new("add")
                .about("Add file path to repository")
                .arg(
                    Arg::new("PATH")
                        .about("Given an existed file path")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("name")
                        .about("Specify file name")
                        .long("--name")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("unlink")
                .about("Unlink a dotfile from current repository")
                .arg(
                    Arg::new("NAME")
                        .about("Given a dotfile name")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("info")
                .about("Show a dotfile detail from current repository")
                .arg(
                    Arg::new("NAME")
                        .about("Given a dotfile name")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                ),
        )
}

fn get_cwd() -> String {
    let path = env::current_dir().unwrap();
    path.display().to_string()
}
