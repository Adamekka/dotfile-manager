mod lib;

use lib::set_folders;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
struct Config {
    name: Option<String>,
    path: Option<String>,
    git_path: Option<String>,
}

pub fn import(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let config_folder = set_folders();

    let config = Config {
        name: name,
        path: path,
        git_path: git_path,
    };

    write_config(config, config_folder);
}

fn write_config(config: Config, config_folder: String) {
    let template_folder = config_folder.clone() + "/templates";
    let template_folder_path = Path::new(&template_folder);

    // Create templates folder
    if !template_folder_path.exists() {
        fs::create_dir(template_folder_path)
            .expect("Can't create '~/.config/dotfile-manager/templates/")
    }

    // Create file contents
    let toml = toml::to_string(&config).unwrap();

    // Create file path
    let config_path = template_folder + "/" + &config.name.unwrap() + ".toml";

    // Write file to fs
    let result = fs::write(config_path, toml);

    // Print result
    println!("{:?}", result);
}
