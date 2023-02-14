use serde::Deserialize;
use std::{
    env,
    fs::{self, ReadDir},
    path::Path,
};
use tabled::Tabled;

#[derive(Clone, Debug, Default, Deserialize, Tabled)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub git_path: String,
}

#[derive(Debug, Deserialize)]
struct Toml {
    template: Template,
}

#[macro_export]
macro_rules! pretty_panic {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        print!("{}", "Error: ".red().bold());
        println!("{}", format!($msg));
        #[cfg(debug_assertions)]
        panic!();
        #[cfg(not(debug_assertions))]
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        print!("{}", "Warning: ".yellow().bold());
        println!("{}", format!($msg));
    };
}

pub fn get_home_folder() -> String {
    #[cfg(target_family = "unix")]
    return env::var("HOME").expect("$HOME environment variable isn't set");

    #[cfg(target_family = "windows")]
    return env::var("USERPROFILE").expect("$USERPROFILE environment variable isn't set");
    #[cfg(target_family = "wasm")]
    pretty_panic!("WebAssembly isn't supported");
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
    let home_folder = get_home_folder();
    let config_folder_path = Path::new(&home_folder).join(".config");

    if !config_folder_path.exists() {
        fs::create_dir(config_folder_path).expect("Can't create '~/.config/'");
    }

    let dman_folder = Path::new(&home_folder).join(".config/dotfile-manager");

    if !dman_folder.exists() {
        fs::create_dir(&dman_folder).expect("Can't create '~/.config/dotfile-manager/");
    }

    // Create fake-git folder
    // This is used to check if remote exists, because Repository::open() need a git repo
    let fake_git_folder = Path::new(&home_folder).join(".local/share/dotfile-manager/fake-git");

    if !fake_git_folder.exists() {
        fs::create_dir_all(&fake_git_folder)
            .expect("Can't create '~/.config/dotfile-manager/fake-git/");

        // check if git is installed
        which::which("git").expect("Git is not installed");
        // git init
        let mut cmd = std::process::Command::new("git");
        cmd.arg("init");
        cmd.current_dir(&fake_git_folder);
        cmd.output().expect("Can't run git init");

        // create readme inside fake-git folder
        let fake_git_readme = "This is a fake git repository, used to check if remote exists";
        fs::write(fake_git_folder.join("readme"), fake_git_readme)
            .expect("Can't write to fake-git folder");
    }

    set_template_folder(&dman_folder)
}

/// Check for template folder, else create one
fn set_template_folder(dman_folder: &Path) -> String {
    let template_folder = Path::new(&dman_folder).join("templates");

    // Create templates folder
    if !template_folder.exists() {
        fs::create_dir(&template_folder)
            .expect("Can't create '~/.config/dotfile-manager/templates/");
    }

    template_folder.to_str().unwrap().to_string()
}

fn get_fake_git_folder() -> String {
    let home_folder = get_home_folder();

    let fake_git_folder = Path::new(&home_folder).join(".local/share/dotfile-manager/fake-git");

    fake_git_folder.to_str().unwrap().to_string()
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
        pretty_panic!("Not enough arguments");
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
        pretty_panic!("Not found");
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
        let answer: Option<question::Answer>;

        answer = question::Question::new($text)
            .yes_no()
            .show_defaults()
            .until_acceptable()
            .ask();

        match answer {
            Some(question::Answer::YES) => {}
            Some(question::Answer::NO) => {
                dotfile_manager::pretty_panic!("Aborting");
            }
            Some(question::Answer::RESPONSE(_)) => {
                unreachable!("Something went wrong");
            }
            None => {
                unreachable!("Something went wrong");
            }
        }
    };
}

/// Check if remote exists
///
/// # Arguments
///
/// * remote: String
///
/// # Panics
///
/// * If remote doesn't exist
///
/// # Example
///
/// ```
/// use dotfile_manager::check_if_remote_exists;
///
/// check_if_remote_exists(String::from("https://github.com/Adamekka/dotfile-manager.git"));
/// ```
///
/// # Debug
///
/// * Print Git path if debug_assertions is set
pub fn check_if_remote_exists(remote: String) {
    let repo = git2::Repository::open(get_fake_git_folder()).unwrap();
    let mut remote = repo
        .find_remote(&remote)
        .or_else(|_| repo.remote_anonymous(&remote))
        .unwrap();
    let connection = remote
        .connect_auth(git2::Direction::Fetch, None, None)
        .unwrap();
    let mut remote_origin_exists = false;
    #[allow(unused_variables)]
    for head in connection.list().unwrap().iter() {
        #[cfg(debug_assertions)]
        println!("{}\t{}", head.oid(), head.name());
        remote_origin_exists = true;
    }

    if remote_origin_exists {
        println!("Remote origin exists");
    } else {
        println!("Remote origin doesn't exist");
    }
}

pub fn get_branches(path: String) -> Vec<String> {
    let path = Path::new(&path);
    let repo = git2::Repository::open(path).expect("Couldn't open repo, bad path maybe?");
    let mut branches = Vec::new();

    for branch in repo.branches(Some(git2::BranchType::Local)).unwrap() {
        let (branch, _) = branch.unwrap();
        let name = branch.name().unwrap().unwrap();
        branches.push(name.to_string());
    }

    branches
}

#[allow(dead_code)]
/// Read templates from filesystem and put them to Vector
pub fn get_templates_to_vec() -> Vec<Template> {
    let templates = get_existing_templates();
    let mut templates_vec: Vec<Template> = Vec::new();

    for template in templates {
        let template = process_template_to_struct(&template);
        templates_vec.push(template);
    }

    dbg!(&templates_vec);

    templates_vec
}
