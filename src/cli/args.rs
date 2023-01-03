#[path = "../import.rs"]
mod import;

use clap::{Arg, ArgAction, ArgMatches, Command};
use import::import;
use std::cfg;

fn arguments() -> ArgMatches {
    Command::new("dman")
        .about("Manage your dotfiles")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            Command::new("import")
                .about("Import a config")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .required(true),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .required(true),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull config from Git repo")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("pull-all").about("Pull all configs from Git repo(s)"))
        .subcommand(
            Command::new("push")
                .about("Push config to Git repo")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("push-all").about("Push all configs to Git repo(s)"))
        .get_matches()
}

// match arguments: import, pull, push, ...
pub fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("import", _set_matches)) => {
            let (name, path, git_path) = match_subcmd_flags("import");
            import(name, path, git_path);
        }

        Some(("pull", _set_matches)) => {
            check_if_enough_flags("pull");
            let (name, path, git_path) = match_subcmd_flags("pull");
        }

        Some(("pull-all", _set_matches)) => {}

        Some(("push", _set_matches)) => {
            check_if_enough_flags("push");
            let (name, path, git_path) = match_subcmd_flags("push");
        }

        Some(("push-all", _set_matches)) => {}

        _ => unreachable!(),
    }
}

// match subcommand flags: -n, -p, -g; --name, --path, --git-path
fn match_subcmd_flags(
    cmd: &str,
) -> (
    Option<std::string::String>,
    Option<std::string::String>,
    Option<std::string::String>,
) {
    let args = arguments();

    let mut name: Option<String> = None;
    let mut path: Option<String> = None;
    let mut git_path: Option<String> = None;

    if let Some(arg_match) = args.subcommand_matches(cmd) {
        if arg_match.get_one::<String>("name") != None {
            name = Some(arg_match.get_one::<String>("name").unwrap().to_string());
        }

        if arg_match.get_one::<String>("path") != None {
            path = Some(arg_match.get_one::<String>("path").unwrap().to_string());
        }

        if arg_match.get_one::<String>("git-path") != None {
            git_path = Some(arg_match.get_one::<String>("git-path").unwrap().to_string());
        }
    } else {
        // this never gets called
        panic!("Clap somehow screwed up");
    }

    #[cfg(debug_assertions)]
    {
        println!("{:?}, {:?}, {:?}", name, path, git_path);
    }

    return (name, path, git_path);
}

// Check if at least 1 flag is present
fn check_if_enough_flags(cmd: &str) {
    let args = arguments();

    if let Some(arg_match) = args.subcommand_matches(cmd) {
        if !(arg_match.get_one::<String>("name") != None
            || arg_match.get_one::<String>("path") != None
            || arg_match.get_one::<String>("git-path") != None)
        {
            panic!("At least 1 flag is required");
        }
    }
}
