#[path = "./libgit2-rs/pull.rs"]
mod pull;

use crate::lib;
use core::panic;
use lib::{get_existing_templates, process_template_to_struct, Template};
use std::{cfg, fs};

pub fn pull(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let templates = get_existing_templates();

    // How to match input with saved templates
    let matching: char;

    if name.is_some() {
        println!("Matching by name");
        matching = 'n';
    } else if path.is_some() {
        println!("Matching by path");
        matching = 'p';
    } else if git_path.is_some() {
        println!("Matching by git-path");
        matching = 'g';
    } else {
        panic!("Not enough arguments");
    }

    let mut is_user_input_matched: bool = false;
    let mut template: Template = Default::default();

    // For loop template folder for templates, when matched, pass it to git pull
    for template_file in templates {
        // You need to construct template_temp struct every time in a loop, because you wanna for loop your existing templates in your fs
        // You can't use template variable, because it's constructed after templates_temp values are matched with user input and then passed to git pull
        let template_temp = process_template_to_struct(&template_file);

        match matching {
            // name
            'n' => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.name,
                    &name,
                    template_file,
                );
            }
            // path
            'p' => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.path,
                    &path,
                    template_file,
                );
            }
            // git-path
            'g' => {
                (is_user_input_matched, template) = match_user_input_with_template_data(
                    is_user_input_matched,
                    template_temp.git_path,
                    &git_path,
                    template_file,
                );
            }
            _ => {
                panic!("Match error");
            }
        }
    }
    if !is_user_input_matched {
        println!("Not found");
    } else {
        #[cfg(debug_assertions)]
        {
            println!("{:?}", template);
        }

        // Pass path from matched template to function, that'll pull changes from GitHub
        let result = pull::run(template.path);
        println!("{:?}", result);
    }
}

/// Git pull every template
pub fn pull_all() {
    let templates = get_existing_templates();

    for template_file in templates {
        let template = process_template_to_struct(&template_file);

        println!("Pulling changes for: {}", template.name);
        #[cfg(debug_assertions)]
        {
            println!("{:?}", template);
        }
        let result = pull::run(template.path);
        println!("{:?}", result);
    }
}

/// Match user input with existing templates to find one to Git pull
fn match_user_input_with_template_data(
    previous_value: bool,
    template_data: String,
    user_input: &Option<String>,
    template_file: Result<fs::DirEntry, std::io::Error>,
) -> (bool, Template) {
    let user_input = user_input.clone().unwrap();

    // Construct template according to data and return it
    let template = process_template_to_struct(&template_file);

    if template_data == user_input {
        println!("{} template found", template.name);

        (true, template)
    } else {
        (previous_value, template)
    }
}
