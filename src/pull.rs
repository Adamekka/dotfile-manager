#[path = "./libgit2-rs/pull.rs"]
mod pull;

use crate::lib;
use core::panic;
use lib::match_user_input_with_existing_templates;
use lib::{get_existing_templates, process_template_to_struct, Template};
use std::{cfg, fs};

pub fn pull(name: Option<String>, path: Option<String>, git_path: Option<String>) {
    let template = match_user_input_with_existing_templates(name, path, git_path);

    // Pass path from matched template to function, that'll pull changes from GitHub
    let result = pull::run(template.path);
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
        let result = pull::run(template.path);
        println!("{result:?}");
    }
}
