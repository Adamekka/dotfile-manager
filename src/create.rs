#[path = "./libgit2-rs/clone_git.rs"]
mod clone_git;

use crate::lib;
use dotfile_manager::{pretty_panic, question_yes_no};
use lib::{check_if_remote_exists, set_folders};
use serde::Serialize;
use std::{env, fs, path::Path};

#[derive(Clone, Serialize)]
struct Template {
    name: Option<String>,
    path: Option<String>,
    git_path: Option<String>,
}

#[derive(Serialize)]
struct Toml {
    template: Template,
}

/// Construct a struct with template parameters
///
/// # Arguments
///
/// * `name` - Name of the template
/// * `path` - Path to the template
/// * `git_path` - Path to the git repository
///
/// # Panics
///
/// * If template already exists
/// * If path does not exist
/// * If git path does not exist
///
/// # Examples
///
/// ```
/// use dotfile_manager::create_template;
///
/// create_template(Some("test".to_string()), Some("/home/user/test".to_string()), Some("https://github.com/user/repository".to_string()));
/// ```
pub fn create_template(name: Option<String>, path: Option<String>, git_path: Option<String>) {
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
    // This is needed because i want toml to have table name
    // [template]
    // name = "..."
    // path = "..."
    // git_path = "..."
    let mut template = Toml { template };

    // Create file contents
    let mut toml = toml::to_string(&template).unwrap();

    // Replace ~ with home path
    // this is needed because ~ is not expanded by the std::path::Path
    // and the toml crate does not expand it either
    let home = env::var("HOME").expect("$HOME environment variable isn't set");
    toml = toml.replace('~', home.as_str());

    // Create file path
    let template_path_string =
        template_folder.clone() + "/" + template.template.name.as_ref().unwrap() + ".toml";
    let template_path = Path::new(&template_path_string);

    // Check if template already exists
    if template_path.exists() {
        pretty_panic!("Same path template already exists");
    }

    // Check if path defined in template exists
    let mut tmp = template.template.path.as_mut().unwrap().clone();
    // Replace ~ with home path
    // this is needed because ~ is not expanded by the std::path::Path
    // and the toml crate does not expand it either
    tmp = tmp.replace('~', home.as_str());

    let path_in_template = Path::new(&tmp);
    if !path_in_template.exists() {
        println!("Path {path_in_template:?} does not exist");
        question_yes_no!("Do you want to clone this template from Git repository?");

        let result = clone_git::run(
            template.template.git_path.as_ref().unwrap().as_str(),
            path_in_template,
        );

        match result {
            Ok(_) => {
                println!("Cloned template from Git repository");

                // Repeat this function
                write_template_to_fs(template.template.clone(), template_folder);

                return;
            }
            Err(error) => {
                println!("Error: {error:?}");
                pretty_panic!("Could not clone template from Git repository");
            }
        }
    } else if path_in_template.is_file() {
        pretty_panic!("Path is a file");
    } else if path_in_template.is_dir() {
        // Check if folder is empty
        if path_in_template.read_dir().unwrap().next().is_none() {
            println!("Folder {path_in_template:?} exists but is empty");
            question_yes_no!("Clone into it?");

            let result = clone_git::run(
                template.template.git_path.as_ref().unwrap().as_str(),
                path_in_template,
            );

            match result {
                Ok(_) => {
                    println!("Cloned template from Git repository");

                    // Repeat this function
                    write_template_to_fs(template.template.clone(), template_folder);

                    return;
                }
                Err(error) => {
                    println!("Error: {error:?}");
                    pretty_panic!("Could not clone template from Git repository");
                }
            }
        }
    }

    // Check if path defined in template is a git repository
    let repo = match git2::Repository::open(path_in_template) {
        Ok(repo) => repo,
        Err(_) => {
            // Check if folder is empty
            if !path_in_template.read_dir().unwrap().next().is_none() {
                pretty_panic!(
                    "Path: {path_in_template:?} isn't empty, make sure it's empty before cloning"
                );
            }

            pretty_panic!("Path: {path_in_template:?} is not a git repository");
        }
    };

    match repo.find_remote("origin") {
        Ok(_) => {
            println!("Path: Remote origin exists");
        }
        Err(_) => {
            pretty_panic!("Path: Remote origin does not exist");
        }
    }

    // Check if git path defined in template exists
    check_if_remote_exists(template.template.git_path.unwrap());

    // Write template to fs ~/.config/dotfile-manager/templates/foo.toml
    let result = fs::write(template_path, toml);

    // Print result
    println!("{result:?}");
}
