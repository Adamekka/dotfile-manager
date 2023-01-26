use serde::Deserialize;
use std::{
    env,
    fs::{self, ReadDir},
    path::Path,
};
use tabled::Tabled;

#[derive(Debug, Default, Deserialize, Tabled)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub git_path: String,
}

/// Check for config folder, else create one
/// Same for dotfile-manager folder
pub fn set_folders() -> String {
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

/// Check for template folder, else create one
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

/// Get templates from filesystem ~/.config/templates/
pub fn get_existing_templates() -> ReadDir {
    let template_folder = set_folders();

    // Get templates from template folder
    fs::read_dir(template_folder).unwrap()
}

/// Process file to Template struct
pub fn process_template_to_struct(file: &Result<fs::DirEntry, std::io::Error>) -> Template {
    let template_but_string = fs::read_to_string(file.as_ref().unwrap().path()).unwrap();
    let template: Template = toml::from_str(&template_but_string).expect("Couldn't parse");

    #[cfg(debug_assertions)]
    {
        println!("{template:?}");
    }

    template
}
