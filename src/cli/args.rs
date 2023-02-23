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
#[path = "../updater.rs"]
mod updater;

use crate::lib;
use clap::{Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};
use create::create_template;
use dotfile_manager::pretty_panic;
use export::export_templates;
use import::import_templates;
use lib::get_home_folder;
use list::list_templates;
use pull::{pull, pull_all};
use remove::remove_template;
use std::path::Path;
use updater::{check_updates, update};

/// Get arguments from Clap
fn arguments() -> Command {
    Command::new("dman")
        .about("Manage your dotfiles")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            Command::new("new")
                .about("Create new template")
                .visible_alias("create")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .required(true)
                        .value_hint(clap::ValueHint::Unknown)
                        .help("Name of the template"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .required(true)
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Path to the template"),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .required(true)
                        .value_hint(clap::ValueHint::Url)
                        .help("Git path to the template"),
                ),
        )
        .subcommand(Command::new("list").about("List all templates"))
        .subcommand(
            Command::new("import")
                .about("Import template(s) from toml file")
                .arg(
                    Arg::new("file")
                        .required(true)
                        .value_hint(clap::ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("export")
                .about("Export template(s) to toml file")
                .arg(
                    Arg::new("file")
                        .required(false)
                        .value_hint(clap::ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove template from dman, not from filesystem")
                .arg(Arg::new("template").required(false))
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Unknown)
                        .help("Name of the template"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Path to the template"),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Url)
                        .help("Git path to the template"),
                ),
        )
        .subcommand(
            Command::new("pull")
                .about("Clone template and pull changes from remote")
                .arg(
                    Arg::new("template")
                        .required(false)
                        .value_hint(clap::ValueHint::Unknown),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Unknown)
                        .help("Name of the template"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Path to the template"),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Url)
                        .help("Git path to the template"),
                ),
        )
        .subcommand(
            Command::new("pull-all").about("Clone all templates and pull changes from remote"),
        )
        .subcommand(
            Command::new("push")
                .about("Push changes to Git repo")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Unknown)
                        .help("Name of the template"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Path to the template"),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .value_hint(clap::ValueHint::Url)
                        .help("Git path to the template"),
                ),
        )
        .subcommand(Command::new("check-updates").about("Check for updates"))
        .subcommand(Command::new("update").about("Update dman to the latest version"))
}

fn print_completions<G: Generator + std::marker::Copy>(
    gen: G,
    cmd: &mut Command,
    shell: clap_complete::Shell,
) {
    // Binding is needed because of lifetime
    let binding = get_home_folder();
    let home = Path::new(&binding);

    // Create directory for completion file if it doesn't exist
    let dir = match shell {
        clap_complete::Shell::Bash => home.join(".local/share/bash-completion/completions"),
        clap_complete::Shell::Fish => home.join(".config/fish/completions"),
        clap_complete::Shell::Zsh => todo!("Zsh completions not implemented yet"),
        // clap_complete::Shell::Zsh => home.join(".local/share/zsh/site-functions"),
        clap_complete::Shell::Elvish => todo!("Elvish completions not implemented yet"),
        // clap_complete::Shell::Elvish => home.join(".local/share/elvish/site-functions"),
        _ => panic!("Shell not supported"),
    };

    std::fs::create_dir_all(dir).expect("Failed to create directory for shell completion file");

    // Get path to completion file
    let path = match shell {
        clap_complete::Shell::Bash => home.join(".local/share/bash-completion/completions/dman"),
        clap_complete::Shell::Fish => home.join(".config/fish/completions/dman.fish"),
        // clap_complete::Shell::Zsh => home.join(".local/share/zsh/site-functions/dman"),
        // clap_complete::Shell::Elvish => home.join(".local/share/elvish/site-functions/_dman.elv"),
        _ => panic!("Shell not supported"),
    };

    let mut file = std::fs::File::create(path).expect("Failed to create shell completion file");
    generate(gen, cmd, cmd.get_name().to_string(), &mut file);
}

/// Match arguments: new, pull, push, ...
/// Then pass them to according function with their parameters
pub fn match_args() {
    let args = arguments().get_matches();

    // Generate completion file
    let shell = get_shell::get_shell().expect("Failed to get shell");
    let generator = match shell {
        get_shell::Shell::Bash => Shell::Bash,
        get_shell::Shell::Fish => Shell::Fish,
        get_shell::Shell::Zsh => Shell::Zsh,
        get_shell::Shell::Elvish => Shell::Elvish,
        _ => panic!("Shell not supported"),
    };

    #[cfg(debug_assertions)]
    println!("Generating completion file for {generator}...");
    print_completions(generator, &mut arguments(), generator);

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

        Some(("check-updates", _set_matches)) => {
            check_updates();
        }

        Some(("update", _set_matches)) => {
            update();
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
    let args = arguments().get_matches();

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
    let args = arguments().get_matches();

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
    let args = arguments().get_matches();

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
