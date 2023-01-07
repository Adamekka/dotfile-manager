mod lib;
#[path = "./libgit2-rs/pull.rs"]
mod pull;

use core::panic;
use lib::set_folders;
use serde::Deserialize;
use std::{
    cfg,
    fs::{self, ReadDir},
};

#[derive(Debug, Default, Deserialize)]
struct SavedConfig {
    name: String,
    path: String,
    git_path: String,
}

fn setup() -> ReadDir {
    let template_folder = set_folders();

    // Get files from template folder
    return fs::read_dir(template_folder).unwrap();
}

pub fn pull(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let files = setup();

    // How to match input with saved configs
    let matching: char;

    if name.is_some() {
        println!("Matching by name");
        matching = 'n';
    } else if path.is_some() {
        println!("Matching by path");
        matching = 'p';
    } else if git_path.is_some() {
        println!("Matching by git-path");
        matching = 'g';
    } else {
        panic!("Not enough arguments");
    }

    let mut are_data_matched: bool = false;
    let mut config: SavedConfig = Default::default();

    // For loop template folder for files
    for file in files {
        let saved_config = process_file_to_struct(&file);

        match matching {
            // name
            'n' => {
                (are_data_matched, config) =
                    match_data(are_data_matched, saved_config.name, &name, file);
            }
            // path
            'p' => {
                (are_data_matched, config) =
                    match_data(are_data_matched, saved_config.path, &path, file);
            }
            // git-path
            'g' => {
                (are_data_matched, config) =
                    match_data(are_data_matched, saved_config.git_path, &git_path, file);
            }
            _ => {
                panic!("Match error");
            }
        }
    }
    if !are_data_matched {
        println!("Not found");
    } else {
        let result = pull::run(config.path);
        println!("{:?}", result);
    }
}

pub fn pull_all() {
    let files = setup();
}

fn process_file_to_struct(file: &Result<fs::DirEntry, std::io::Error>) -> SavedConfig {
    let text = fs::read_to_string(file.as_ref().unwrap().path());
    let text_string = text.unwrap();
    let saved_config: SavedConfig = toml::from_str(&text_string).expect("Couldn't parse");

    #[cfg(debug_assertions)]
    {
        println!("{:?}", saved_config);
    }

    saved_config
}

fn match_data(
    previous_value: bool,
    saved_config: String,
    data: &Option<String>,
    file: Result<fs::DirEntry, std::io::Error>,
) -> (bool, SavedConfig) {
    let data = data.clone().unwrap();
    let saved_config_struct = process_file_to_struct(&file);

    if saved_config == data {
        println!("{} found", saved_config_struct.name);

        return (true, saved_config_struct);
    } else {
        return (previous_value, saved_config_struct);
    }
}
