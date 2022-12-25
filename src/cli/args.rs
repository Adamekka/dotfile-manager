use clap::{Arg, ArgAction, ArgMatches, Command};

// TODO: pull, push - set number of required args to 1
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
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
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
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull config from Git repo")
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("pull-all").about("Pull all configs from Git repo(s)"))
        .subcommand(
            Command::new("push")
                .about("Push config to Git repo")
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("push-all").about("Push all configs to Git repo(s)"))
        .get_matches()
}

pub fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("import", set_matches)) => {
            match_subcmd_flags("import");
        }
        Some(("pull", set_matches)) => {
            match_subcmd_flags("import");
        }
        Some(("pull-all", set_matches)) => {}
        Some(("push", set_matches)) => {
            match_subcmd_flags("import");
        }
        Some(("push-all", set_matches)) => {}
        _ => unreachable!(),
    }
}

fn match_subcmd_flags(cmd: &str) {
    let args = arguments();
    if let Some(cmd) = args.subcommand_matches(cmd) {
        let git_path = cmd.get_one::<String>("git-path").map(|s| s.as_str());
        let path = cmd.get_one::<String>("path").map(|s| s.as_str());
        let name = cmd.get_one::<String>("name").map(|s| s.as_str());

        println!("git-path:{:?}, path:{:?}, name:{:?}", git_path, path, name);
    }
}
