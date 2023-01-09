use serde::Deserialize;
use std::{
    env,
    fs::{self, ReadDir},
    path::Path,
};

#[derive(Debug, Default, Deserialize)]
pub struct SavedConfig {
    pub name: String,
    pub path: String,
    pub git_path: String,
}

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

pub fn get_files() -> ReadDir {
    let template_folder = set_folders();

    // Get files from template folder
    fs::read_dir(template_folder).unwrap()
}

pub fn process_file_to_struct(file: &Result<fs::DirEntry, std::io::Error>) -> SavedConfig {
    let text = fs::read_to_string(file.as_ref().unwrap().path());
    let text_string = text.unwrap();
    let saved_config: SavedConfig = toml::from_str(&text_string).expect("Couldn't parse");

    #[cfg(debug_assertions)]
    {
        println!("{:?}", saved_config);
    }

    saved_config
}
