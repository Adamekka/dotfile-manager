#[path = "../lib.rs"]
mod lib;

use clap::{Arg, ArgMatches, Command};

fn main() {
    match_args();
}

fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("import", set_matches)) => {}
        Some(("pull", set_matches)) => {}
        Some(("pull-all", set_matches)) => {}
        Some(("push", set_matches)) => {}
        Some(("push-all", set_matches)) => {}
        _ => unreachable!(),
    }
}

fn arguments() -> ArgMatches {
    Command::new("dman")
        .about("Manage your dotfiles")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            Command::new("import")
                .about("Import a config")
                .arg(Arg::new("git-path").short('g').long("git-path"))
                .arg(Arg::new("path").short('p').long("path"))
                .arg(Arg::new("name").short('n').long("name")),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull config from Git repo")
                .arg(Arg::new("git-path").short('g').long("git-path"))
                .arg(Arg::new("path").short('p').long("path"))
                .arg(Arg::new("name").short('n').long("name")),
        )
        .subcommand(Command::new("pull-all").about("Pull all configs from Git repo(s)"))
        .subcommand(
            Command::new("push")
                .about("Push config to Git repo")
                .arg(Arg::new("git-path").short('g').long("git-path"))
                .arg(Arg::new("path").short('p').long("path"))
                .arg(Arg::new("name").short('n').long("name")),
        )
        .subcommand(Command::new("push-all").about("Push all configs to Git repo(s)"))
        .get_matches()
}
