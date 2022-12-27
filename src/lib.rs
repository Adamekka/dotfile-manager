use std::{env, fs, path::Path};

#[allow(dead_code)] // rust-analyzer doesn't know it's called in main()
pub fn set_folders() {
    // Check for config folder, else create one
    let home_folder = env::var("HOME").expect("$HOME environment variable isn't set");
    let config_folder = home_folder.clone() + "/.config/dotfile-manager";
    let config_folder = Path::new(&config_folder);

    if !config_folder.exists() {
        fs::create_dir(config_folder).expect("Can't create '~/.config/dotfile-manager/")
    }
}
