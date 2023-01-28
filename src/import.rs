mod lib;

use lib::Template;
use std::path::Path;

/// Import templates from a file
pub fn import_templates(file_path: String) {
    let file_path = Path::new(&file_path);

    // check if file exists
    if !file_path.exists() {
        panic!("File does not exist");
    }

    // check if file is not directory
    if file_path.is_dir() {
        panic!("File is directory and not file");
    }

    // check if file is a toml file
    match file_path.extension() {
        Some(extension) => match extension.to_str() {
            Some(extension) => {
                if extension != "toml" {
                    panic!("File is not a toml file");
                }
            }
            None => {
                panic!("File is not a toml file");
            }
        },
        None => {
            panic!("File is not a toml file");
        }
    }

    let file = std::fs::read_to_string(file_path).unwrap();
    let file_contents: toml::Value = toml::from_str(&file).unwrap();
    let mut templates: Vec<Template> = Vec::new();

    // put templates to vector
    for (key, value) in file_contents.as_table().unwrap().iter() {
        #[cfg(debug_assertions)]
        {
            println!("{key}: {value}");
        }

        // check if file contains valid values

        let name = match value.get("name") {
            Some(name) => name,
            None => {
                panic!("Template does not contain a name");
            }
        };

        let path = match value.get("path") {
            Some(path) => path,
            None => {
                panic!("Template does not contain a path");
            }
        };

        let git_path = match value.get("git_path") {
            Some(git_path) => git_path,
            None => {
                panic!("Template does not contain a git_path");
            }
        };

        #[cfg(debug_assertions)]
        {
            println!("name: {name}");
            println!("path: {path}");
            println!("git_path: {git_path}");
        }

        let template = Template {
            name: name.to_string(),
            path: path.to_string(),
            git_path: git_path.to_string(),
        };

        templates.push(template);
    }

    #[cfg(debug_assertions)]
    {
        println!("templates: {templates:?}");
    }

    // check if template(s) already exists
    // import template(s)

    println!("Importing templates...");
}
