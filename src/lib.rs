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

#[derive(Debug, Deserialize)]
struct Toml {
    template: Template,
}

/// Check for config folder, else create one
/// Same for dotfile-manager folder
///
/// Returns the path to the template folder
///
/// # Panics
///
/// * If $HOME environment variable isn't set
/// * If ~/.config/ can't be created
/// * If ~/.config/dotfile-manager/ can't be created
/// * If ~/.config/dotfile-manager/templates/ can't be created
///
/// # Example
///
/// ```
/// use dotfile_manager::set_folders;
///
/// let template_folder = set_folders();
/// ```
pub fn set_folders() -> String {
    #[cfg(target_os = "unix")]
    let home_folder = env::var("HOME").expect("$HOME environment variable isn't set");
    #[cfg(target_os = "windows")]
    let home_folder = env::var("USERPROFILE").expect("$USERPROFILE environment variable isn't set");
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
///
/// # Panics
///
/// * If template folder can't be read
///
/// # Example
///
/// ```
/// use dotfile_manager::get_existing_templates;
///
/// let templates = get_existing_templates();
/// ```
pub fn get_existing_templates() -> ReadDir {
    let template_folder = set_folders();

    // Get templates from template folder
    fs::read_dir(template_folder).unwrap()
}

/// Process file to Template struct
///
/// # Arguments
///
/// * file: &Result<fs::DirEntry, std::io::Error>
///
/// # Panics
///
/// * If template can't be parsed
///
/// # Example
///
/// ```
/// use dotfile_manager::process_template_to_struct;
/// use dotfile_manager::get_existing_templates;
///
/// let templates = get_existing_templates();
///
/// // Iterate over templates
///    for template in templates {
/// let template = process_template_to_struct(&template);
/// }
/// ```
/// # Debug
///
/// * Print template if debug_assertions is set
pub fn process_template_to_struct(file: &Result<fs::DirEntry, std::io::Error>) -> Template {
    let template_but_string = fs::read_to_string(file.as_ref().unwrap().path()).unwrap();
    let template: Toml = toml::from_str(&template_but_string).expect("Couldn't parse");
    // This is needed because I need to return clean Template struct, not Toml struct
    // Toml struct contains Template struct inside
    let template = template.template;

    #[cfg(debug_assertions)]
    {
        println!("{template:?}");
    }

    template
}

enum Matching {
    Name,
    Path,
    GitPath,
}

pub fn match_user_input_with_existing_templates(
    name: Option<String>,
    path: Option<String>,
    git_path: Option<String>,
) -> Template {
    let templates = get_existing_templates();

    // How to match input with saved templates
    let matching: Matching;

    if name.is_some() {
        println!("Matching by name");
        matching = Matching::Name;
    } else if path.is_some() {
        println!("Matching by path");
        matching = Matching::Path;
    } else if git_path.is_some() {
        println!("Matching by git-path");
        matching = Matching::GitPath;
    } else {
        panic!("Not enough arguments");
    }

    let mut is_user_input_matched: bool = false;
    let mut template: Template = Default::default();

    // For loop template folder for templates, when matched, return it
    for template_file in templates {
        // You need to construct template_temp struct every time in a loop, because you wanna for loop your existing templates in your fs
        // If you use template variable, it'll be empty, because it's constructed after templates_temp values are matched with user input and then returned
        let template_temp = process_template_to_struct(&template_file);

        match matching {
            Matching::Name => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.name,
                    &name,
                    template_file,
                    template,
                );
            }
            Matching::Path => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.path,
                    &path,
                    template_file,
                    template,
                );
            }
            Matching::GitPath => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.git_path,
                    &git_path,
                    template_file,
                    template,
                );
            }
        }
    }
    if !is_user_input_matched {
        panic!("Not found");
    } else {
        #[cfg(debug_assertions)]
        {
            println!("Returning template: ");
            dbg!(&template);
        }

        template
    }
}

/// Match user input with existing templates to find one to Git pull
fn match_user_input_with_template_data(
    previous_value: bool,
    template_data: String,
    user_input: &Option<String>,
    template_file: Result<fs::DirEntry, std::io::Error>,
    previous_matched_template: Template,
) -> (bool, Template) {
    let user_input = user_input.clone().unwrap();

    // Construct template according to data and return it
    let template = process_template_to_struct(&template_file);

    if template_data == user_input {
        println!("{} template found", template.name);

        (true, template)
    } else {
        (previous_value, previous_matched_template)
    }
}

#[macro_export]
macro_rules! question_yes_no {
    ($text:expr) => {
        let answer: Option<Answer>;

        answer = question::Question::new($text)
            .yes_no()
            .show_defaults()
            .until_acceptable()
            .ask();

        match answer {
            Some(Answer::YES) => {}
            Some(Answer::NO) => {
                panic!("Aborting");
            }
            Some(Answer::RESPONSE(_)) => {
                unreachable!("Something went wrong");
            }
            None => {
                unreachable!("Something went wrong");
            }
        }
    };
}
