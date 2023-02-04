use crate::args;
use crate::lib;
use args::create_template;
use core::panic;
use lib::{get_existing_templates, process_template_to_struct, Template};
use std::path::Path;

/// Import templates from a file
///
/// # Arguments
///
/// * `file_path` - Path to the file
///
/// # Panics
///
/// * If the file does not exist
/// * If the file is a directory
/// * If the file is not a toml file
/// * If the file does not contain a name
/// * If the file does not contain a path
/// * If the file does not contain a git_path
///
/// # Examples
///
/// ```
/// use dotfile_manager::import_templates;
///
/// import_templates(String::from("/home/user/.config/dotfile-manager/templates.toml"));
/// ```
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

        // check if file contains table name
        // otherwise it would use wrong key as table name and couldn't find the values

        match key.as_str() {
            "name" | "path" | "git_path" => {
                panic!("Table name missing");
            }
            _ => {}
        }

        // check if file contains valid values
        // if not, panic
        // if yes, put them in a vector
        // and then add them to the templates vector

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

        // this is needed, because the values are wrapped in quotes
        // and we need to remove them
        // they look like this: "value"
        // and we need to remove the quotes
        // so they look like this: value
        // and we can use them
        // this is a workaround, because I don't know how to do it better
        // if you know how to do it better, please open an issue on GitHub
        // or make a pull request

        let mut name = name.to_string();
        name.pop();
        name.remove(0);

        let mut path = path.to_string();
        path.pop();
        path.remove(0);

        let mut git_path = git_path.to_string();
        git_path.pop();
        git_path.remove(0);

        let template = Template {
            name,
            path,
            git_path,
        };

        // check if template(s) already exists
        let existing_templates = get_existing_templates();
        let mut is_template_already_existing = false;
        for existing_template in existing_templates {
            let existing_template = process_template_to_struct(&existing_template);

            #[cfg(debug_assertions)]
            {
                println!("{:?}", existing_template.name);
                println!("{:?}", template.name);
            }

            if existing_template.name == template.name {
                // If template already exists, skip it
                println!("Template {:?} already exists, skipping..", template.name);
                is_template_already_existing = true;
            }
        }

        if !(is_template_already_existing) {
            templates.push(template);
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("templates: {templates:?}");
    }

    println!("Importing templates...");

    // import template(s)
    for template in templates {
        println!("Importing template {:?}", template.name);
        create_template(
            Some(template.name),
            Some(template.path),
            Some(template.git_path),
        );
    }
}
