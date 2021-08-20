use std::env;
use std::process;

extern crate clap;
use clap::{App, Arg};

use crate::commands::*;
use crate::show_error;

pub fn run() {
    let app = init_app();
    let cmd_name;
    let cmd: Box<dyn Command>;
    let cwd = get_cwd();
    let path = format!("{}/dottie.toml", cwd);

    let matches = app.get_matches();
    if let Some(ref _matches) = matches.subcommand_matches("ls") {
        cmd_name = "ls";
        cmd = Box::new(ls::ListOpt::new(&path));
    } else if let Some(ref matches) = matches.subcommand_matches("info") {
        cmd_name = "info";
        let name = matches.value_of("NAME").unwrap_or("");
        cmd = Box::new(info::InfoOpt::new(&path, &name));
    } else if let Some(ref matches) = matches.subcommand_matches("clone") {
        cmd_name = "clone";
        let name = matches.value_of("NAME").unwrap_or("");
        cmd = Box::new(clone::CloneOpt::new(name));
    } else if let Some(ref matches) = matches.subcommand_matches("init") {
        cmd_name = "init";
        let git_repo = matches.value_of("git").unwrap_or("");
        cmd = Box::new(init::InitOpt::new(git_repo.to_string()));
    } else if let Some(ref matches) = matches.subcommand_matches("add") {
        cmd_name = "add";
        let name = matches.value_of("name").unwrap_or("");
        let src = matches.value_of("PATH").unwrap_or("");
        cmd = Box::new(add::AddOpt::new(&path, &name, &src));
    // } else if let Some(ref matches) = matches.subcommand_matches("link") {
    } else if let Some(ref matches) = matches.subcommand_matches("unlink") {
        cmd_name = "unlink";
        let name = matches.value_of("name").unwrap_or("");
        cmd = Box::new(unlink::UnlinkOpt::new(name.to_string()));
    } else {
        return;
    }

    if let Err(_e) = cmd.as_ref().run() {
        // each command should show its own error message
        show_error!("Running command `{}` failed", cmd_name);
        process::exit(1);
    }
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
