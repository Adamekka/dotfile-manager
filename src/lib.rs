use std::{env, fs, path::Path};

pub fn set_folders() -> String {
    // Check for config folder, else create one
    let home_folder = env::var("HOME").expect("$HOME environment variable isn't set");
    let config_folder = home_folder.clone() + "/.config/dotfile-manager";
    let config_folder_path = Path::new(&config_folder);

    if !config_folder_path.exists() {
        fs::create_dir(config_folder_path).expect("Can't create '~/.config/dotfile-manager/")
    }

    let template_folder = set_template_folder(&config_folder);

    template_folder
}

fn set_template_folder(config_folder: &String) -> String {
    let template_folder = config_folder.clone() + "/templates";
    let template_folder_path = Path::new(&template_folder);

    // Create templates folder
    if !template_folder_path.exists() {
        fs::create_dir(template_folder_path)
            .expect("Can't create '~/.config/dotfile-manager/templates/")
    }

    template_folder
}
