use crate::args::{create::clone_git, pull::pull_git};
use git2::Repository;
use mytools::{env::get_home_folder, pretty_panic, question_yes_no};
use question::Question;
use std::{path::Path, process::Command};

fn clone_dman_repo() -> String {
    let binding = &get_home_folder();
    let home = Path::new(binding);
    let dman_repo_path = home.join(".local/share/dotfile-manager/src");
    let dman_repo_remote = "https://github.com/Adamekka/dotfile-manager";

    if !dman_repo_path.exists() {
        println!("Dotfile-manager repository not found.");
        println!("Cloning dotfile-manager repository...");
        let result = clone_git::run(dman_repo_remote, &dman_repo_path);

        match result {
            Ok(_) => println!("Dotfile-manager repository cloned successfully."),
            Err(e) => {
                pretty_panic!("Failed to clone dotfile-manager repository: {e}");
            }
        }
    }

    dman_repo_path.to_str().unwrap().to_string()
}

pub fn check_updates() {
    println!("Checking for updates...");
    let installed_git_commit_hash = env!("DMAN_GIT_COMMIT_HASH");
    let dman_repo_path = clone_dman_repo();

    // Get remote git commit hash
    let repo =
        Repository::open(&dman_repo_path).expect("Failed to open dotfile-manager repository");
    let mut remote = repo
        .find_remote("origin")
        .expect("Failed to find remote for dotfile-manager repository");
    let connection = remote
        .connect_auth(git2::Direction::Fetch, None, None)
        .unwrap();

    let remote_git_commit_hash = connection
        .list()
        .unwrap()
        .first()
        .unwrap()
        .oid()
        .to_string()
        .chars()
        .take(7)
        .collect::<String>();

    println!("Installed version git commit hash:\t{installed_git_commit_hash}");
    println!("Remote git commit hash:\t\t\t{remote_git_commit_hash}");

    // Compare remote_git_commit_hash with installed_git_commit_hash
    if installed_git_commit_hash == remote_git_commit_hash {
        println!("Dotfile-manager is up to date.");
    } else {
        println!("Dotfile-manager is not up to date.");
        question_yes_no!("Do you want to update dotfile-manager?");
        println!("Updating dotfile-manager...");
        update(dman_repo_path);
    }
}

enum InstallFeatures {
    Cli,
    CliWithGui,
}

fn update(dman_repo_path: String) {
    let remote_branch: Vec<String> = vec!["main".to_string()];
    let result = pull_git::run(dman_repo_path.clone(), remote_branch);

    match result {
        Ok(_) => {}
        Err(e) => {
            pretty_panic!("Failed to update dotfile-manager: {e}");
        }
    }

    // Install new version
    println!("Installing new version...");
    let answer = Question::new("Do you want to install dotfile-manager with gui?")
        .yes_no()
        .show_defaults()
        .until_acceptable()
        .ask()
        .unwrap();

    let install_features = match answer {
        question::Answer::YES => InstallFeatures::CliWithGui,
        question::Answer::NO => InstallFeatures::Cli,
        question::Answer::RESPONSE(_) => unreachable!(),
    };

    Command::new("make")
        .current_dir(dman_repo_path)
        .arg(match install_features {
            InstallFeatures::Cli => "install",
            InstallFeatures::CliWithGui => "install-gui",
        })
        .spawn()
        .expect("Failed to install dotfile-manager")
        .wait()
        .expect("Failed to install dotfile-manager");
}
