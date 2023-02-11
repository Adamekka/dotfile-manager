#[path = "../create.rs"]
mod create;
#[path = "../export.rs"]
mod export;
#[path = "../import.rs"]
mod import;
#[path = "list.rs"]
mod list;
#[path = "../pull.rs"]
mod pull;
#[path = "../remove.rs"]
mod remove;

use clap::{Arg, ArgAction, ArgMatches, Command};
use create::create_template;
use dotfile_manager::pretty_panic;
use export::export_templates;
use import::import_templates;
use list::list_templates;
use pull::{pull, pull_all};
use remove::remove_template;

/// Get arguments from Clap
fn arguments() -> ArgMatches {
    Command::new("dman")
        .about("Manage your dotfiles")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            Command::new("new")
                .about("Create new template")
                .alias("create")
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
        .subcommand(Command::new("list").about("List imported templates"))
        .subcommand(
            Command::new("import")
                .about("Import template(s) from toml file")
                .arg(Arg::new("file").required(true)),
        )
        .subcommand(
            Command::new("export")
                .about("Export template(s) to toml file")
                .arg(Arg::new("file").required(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove template from dman, not from filesystem")
                .arg(Arg::new("template").required(false))
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
        .subcommand(
            Command::new("pull")
                .about("Pull changes from Git repo")
                .arg(Arg::new("template").required(false))
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
        .subcommand(Command::new("pull-all").about("Pull all changes from all Git repo(s)"))
        .subcommand(
            Command::new("push")
                .about("Push changes to Git repo")
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
        .get_matches()
}

/// Match arguments: new, pull, push, ...
/// Then pass them to according function with their parameters
pub fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("new", _set_matches)) => {
            let (name, path, git_path) = match_subcmd_flags("new");
            create_template(name, path, git_path);
        }

        Some(("list", _set_matches)) => {
            list_templates();
        }

        Some(("import", _set_matches)) => {
            let file_path = get_toml_file_from_import();
            import_templates(file_path);
        }

        Some(("export", _arg_matches)) => {
            // Get export_file from arguments
            let args = arguments();
            if let Some(arg_matches) = args.subcommand_matches("export") {
                let export_file = arg_matches.get_one::<String>("file").unwrap().to_string();

                export_templates(export_file);
            }
        }

        Some(("remove", _set_matches)) => {
            check_if_enough_flags("remove");
            let (name, path, git_path) = match_subcmd_flags("remove");
            remove_template(name, path, git_path);
        }

        Some(("pull", _set_matches)) => {
            check_if_enough_flags("pull");
            let (name, path, git_path) = match_subcmd_flags("pull");
            pull(name, path, git_path);
        }

        Some(("pull-all", _set_matches)) => {
            pull_all();
        }

        Some(("push", _set_matches)) => {
            check_if_enough_flags("push");
            let (_name, _path, _git_path) = match_subcmd_flags("push");
            todo!("push");
        }

        _ => unreachable!(),
    }
}

/// Match subcommand flags: -n, -p, -g; --name, --path, --git-path
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
        // If name flag is present use it, otherwise use subcommand argument as Template name
        if arg_match.get_one::<String>("name").is_some() {
            name = arg_match.get_one::<String>("name").cloned();
        } else if arg_match.get_one::<String>("template").is_some() {
            name = arg_match.get_one::<String>("template").cloned();
        }

        if arg_match.get_one::<String>("path").is_some() {
            path = arg_match.get_one::<String>("path").cloned();
        }

        if arg_match.get_one::<String>("git-path").is_some() {
            git_path = arg_match.get_one::<String>("git-path").cloned();
        }
    } else {
        unreachable!("Clap somehow screwed up");
    }

    #[cfg(debug_assertions)]
    {
        println!("{name:?}, {path:?}, {git_path:?}");
    }

    (name, path, git_path)
}

/// Check if at least 1 flag or name of Template is present
/// If not, panic
fn check_if_enough_flags(cmd: &str) {
    let args = arguments();

    if let Some(arg_match) = args.subcommand_matches(cmd) {
        if arg_match.get_one::<String>("name").is_none()
            && arg_match.get_one::<String>("path").is_none()
            && arg_match.get_one::<String>("git-path").is_none()
            && arg_match.get_one::<String>("template").is_none()
        {
            pretty_panic!("At least 1 flag or name of Template is required");
        }
    } else {
        unreachable!("Clap somehow screwed up");
    }
}

/// Get toml file when using import subcommand
fn get_toml_file_from_import() -> String {
    let args = arguments();

    if let Some(arg_match) = args.subcommand_matches("import") {
        if arg_match.get_one::<String>("file").is_some() {
            let file_path = arg_match.get_one::<String>("file").unwrap().to_string();
            println!("{file_path:?}",);

            file_path
        } else {
            pretty_panic!("No file specified");
        }
    } else {
        unreachable!("Clap somehow screwed up");
    }
}
