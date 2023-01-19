use crate::lib;
use lib::set_folders;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
struct Template {
    name: Option<String>,
    path: Option<String>,
    git_path: Option<String>,
}

/// Construct a struct with template parameters
pub fn import(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let template_folder = set_folders();

    let template = Template {
        name,
        path,
        git_path,
    };

    write_template_to_fs(template, template_folder);
}

/// Write template to filesystem
fn write_template_to_fs(template: Template, template_folder: String) {
    // Create file contents
    let toml = toml::to_string(&template).unwrap();

    // Create file path
    let template_path_string = template_folder + "/" + &template.name.unwrap() + ".toml";
    let template_path = Path::new(&template_path_string);

    // Check if template already exists
    if !template_path.exists() {
        panic!("Template already exists");
    }

    // Write template to fs ~/.config/dotfile-manager/templates/foo.toml
    let result = fs::write(template_path, toml);

    // Print result
    println!("{:?}", result);
}
