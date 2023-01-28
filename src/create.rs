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
    // Create file contents
    let toml = toml::to_string(&template).unwrap();

    // Create file path
    let template_path_string = template_folder + "/" + &template.name.unwrap() + ".toml";
    let template_path = Path::new(&template_path_string);

    // Check if template already exists
    if template_path.exists() {
        panic!("Same named template already exists");
    }

    // Check if path defined in template exists
    let tmp = template.path.unwrap();
    let path_in_template = Path::new(&tmp);
    if !path_in_template.exists() {
        panic!("Path {path_in_template:?} does not exist");
    }

    // Check if git path defined in template exists
    let repo = git2::Repository::open(path_in_template).unwrap();
    let remote = repo.find_remote("origin");
    match remote {
        Ok(_) => {
            println!("Remote origin exists");
        }
        Err(_) => {
            panic!("Remote origin does not exist");
        }
    }

    // Write template to fs ~/.config/dotfile-manager/templates/foo.toml
    let result = fs::write(template_path, toml);

    // Print result
    println!("{result:?}");
}
