
use crate::args::create::clone_git;
use crate::lib;
use dotfile_manager::pretty_panic;
use git2::Repository;
use lib::get_home_folder;
use std::path::Path;

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
    let dman_remote_repo =
        Repository::discover(dman_repo_path).expect("Failed to open remote repository");
    // Get remote git commit hash
    let remote_git_commit_hash = dman_remote_repo
        .head()
        .unwrap()
        .peel_to_commit()
        .unwrap()
        .id();

    // Get first 7 characters of remote git commit hash
    // Because the installed git commit hash is also only 7 characters
    let remote_git_commit_hash = remote_git_commit_hash
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
        println!("Updating dotfile-manager...");
        update();
    }

}

pub fn update() {
    todo!();
}
