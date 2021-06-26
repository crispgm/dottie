use std::env;
use std::process;

extern crate clap;
use clap::{App, Arg};

use crate::commands::*;
use crate::show_error;

pub fn run() {
    let app = init_app();

    let matches = app.get_matches();
    let mut err_code: i32 = 0;
    // ls
    if let Some(ref _matches) = matches.subcommand_matches("ls") {
        let cwd = get_cwd();
        let path = format!("{}/dottie.toml", cwd);
        if let Err(e) = ls::ListOpt::new(path).run() {
            show_error!("Running command on {} failed: {}", "ls", e);
            err_code = 1;
        }
    }
    // info
    if let Some(ref matches) = matches.subcommand_matches("info") {
        let cwd = get_cwd();
        let path = format!("{}/fixtures/dottie.toml", cwd);
        let name = matches.value_of("NAME").unwrap_or("");
        if let Err(e) = info::InfoOpt::new(path, name.to_string()).run() {
            show_error!("Running command on {} failed: {}", "info", e);
            err_code = 1;
        }
    }
    // init
    if let Some(ref matches) = matches.subcommand_matches("init") {
        let git_repo = matches.value_of("git").unwrap_or("");
        let init_opt = init::InitOpt::new(git_repo.to_string());
        if let Err(e) = init_opt.run() {
            show_error!("Running command on {} failed: {}", "init", e);
            err_code = 1;
        }
    }
    // add
    if let Some(ref matches) = matches.subcommand_matches("add") {
        let cwd = get_cwd();
        let path = format!("{}/fixtures/dottie.toml", cwd);
        let name = matches.value_of("name").unwrap_or("");
        let src = matches.value_of("PATH").unwrap_or("");
        let add_opt = add::AddOpt::new(path.to_string(), name.to_string(), src.to_string());
        if let Err(e) = add_opt.run() {
            show_error!("Running command on {} failed: {}", "add", e);
            err_code = 1;
        }
    }

    process::exit(err_code);
}

fn init_app() -> App<'static> {
    App::new("dottie")
        .version("1.0.0")
        .author("David Zhang")
        .arg(
            Arg::new("v")
                .about("Sets the level of verbosity")
                .short('v'),
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
            App::new("link")
                .about("Link a dotfile from current repository")
                .arg(
                    Arg::new("NAME")
                        .about("Given a dotfile name")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                )
                .arg(Arg::new("all").about("").long("--all")),
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
