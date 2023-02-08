#[path = "./libgit2-rs/clone_git.rs"]
mod clone_git;

use crate::lib;
use core::panic;
use dotfile_manager::question_yes_no;
use lib::set_folders;
use question::Answer;
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
        panic!("Same named template already exists");
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
                panic!("Could not clone template from Git repository");
            }
        }
    }

    // Check if path defined in template is a git repository
    let repo = git2::Repository::open(path_in_template).unwrap();
    let remote = repo.find_remote("origin");
    match remote {
        Ok(_) => {
            println!("Path: Remote origin exists");
        }
        Err(_) => {
            panic!("Path: Remote origin does not exist");
        }
    }

    // Check if git path defined in template exists
    let repo = git2::Repository::open(".").unwrap();
    let remote = template.template.git_path.unwrap();
    let mut remote = repo
        .find_remote(&remote)
        .or_else(|_| repo.remote_anonymous(&remote))
        .unwrap();
    let connection = remote
        .connect_auth(git2::Direction::Fetch, None, None)
        .unwrap();
    for head in connection.list().unwrap().iter() {
        println!("{}\t{}", head.oid(), head.name());
        println!("Git path: Remote origin exists");
    }

    // Write template to fs ~/.config/dotfile-manager/templates/foo.toml
    let result = fs::write(template_path, toml);

    // Print result
    println!("{result:?}");
}
