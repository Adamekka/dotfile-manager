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
    let template_folder = set_folders();

    let config = Config {
        name,
        path,
        git_path,
    };

    write_config(config, template_folder);
}

fn check_if_already_exists(file_path: &Path) {
    let file_existence = Path::exists(file_path);

    if file_existence {
        panic!("File already exists")
    }
}

fn write_config(config: Config, template_folder: String) {
    // Create file contents
    let toml = toml::to_string(&config).unwrap();

    // Create file path
    let file_path_string = template_folder + "/" + &config.name.unwrap() + ".toml";
    let file_path = Path::new(&file_path_string);

    // Check if file already exists
    check_if_already_exists(file_path);

    // Write file to fs
    let result = fs::write(file_path_string, toml);

    // Print result
    println!("{:?}", result);
}
