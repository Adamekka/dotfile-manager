pub fn check_updates() {
    println!("Checking for updates...");
    let installed_git_commit_hash = std::env::var("DMAN_GIT_COMMIT_HASH").unwrap();
    println!("Installed git commit hash: {installed_git_commit_hash}");
    todo!();
}

pub fn update() {
    todo!();
}
