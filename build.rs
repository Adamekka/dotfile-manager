#[cfg(feature = "gui")]
fn main() {
    add_git_commit_hash();
    tauri_build::build();
}

#[cfg(not(feature = "gui"))]
fn main() {
    add_git_commit_hash();
}

fn add_git_commit_hash() {
    let git_commit_hash = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get git commit hash")
        .stdout;
    let git_commit_hash = String::from_utf8(git_commit_hash).unwrap();
    println!("cargo:rustc-env=DMAN_GIT_COMMIT_HASH={}", git_commit_hash);
}
