#[path = "./libgit2-rs/pull_git.rs"]
mod pull_git;

use crate::lib;
use lib::{
    get_branches, get_existing_templates, match_user_input_with_existing_templates,
    process_template_to_struct,
};

pub fn pull(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let template = match_user_input_with_existing_templates(name, path, git_path);

    let branches = get_branches(template.path.clone());
    // Pass path from matched template to function, that'll pull changes from GitHub
    let result = pull_git::run(template.path, branches);
    println!("{result:?}");
}

/// Git pull every template
pub fn pull_all() {
    let templates = get_existing_templates();

    for template_file in templates {
        let template = process_template_to_struct(&template_file);

        println!("Pulling changes for: {}", template.name);
        #[cfg(debug_assertions)]
        {
            println!("{template:?}");
        }
        let branches = get_branches(template.path.clone());
        let result = pull_git::run(template.path, branches);
        println!("{result:?}");
    }
}
