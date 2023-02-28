use crate::lib;

use lib::{match_user_input_with_existing_templates, set_folders};
use mytools::question_yes_no;

pub fn remove_template(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let template_folder = set_folders();
    let template = match_user_input_with_existing_templates(name, path, git_path);

    question_yes_no!("Are you sure you want to remove this template?");

    println!("Removing template: {}", template.name);

    let template_path = template_folder + "/" + &template.name + ".toml";

    // Remove template folder
    std::fs::remove_file(template_path).unwrap();
}
