use std::{env, fs, path::Path};

pub fn set_folders() -> String {
    // Check for config folder, else create one
    let home_folder = env::var("HOME").expect("$HOME environment variable isn't set");
    let config_folder = home_folder.clone() + "/.config";
    let config_folder_path = Path::new(&config_folder);

    if !config_folder_path.exists() {
        fs::create_dir(config_folder_path).expect("Can't create '~/.config/'");
    }

    let dman_folder = home_folder + "/.config/dotfile-manager";
    let dman_folder_path = Path::new(&dman_folder);

    if !dman_folder_path.exists() {
        fs::create_dir(dman_folder_path).expect("Can't create '~/.config/dotfile-manager/");
    }

    set_template_folder(&dman_folder)
}

fn set_template_folder(dman_folder: &str) -> String {
    let template_folder = dman_folder.to_owned() + "/templates";
    let template_folder_path = Path::new(&template_folder);

    // Create templates folder
    if !template_folder_path.exists() {
        fs::create_dir(template_folder_path)
            .expect("Can't create '~/.config/dotfile-manager/templates/");
    }

    template_folder
}
