use crate::lib::{get_existing_templates, process_template_to_struct};
use mytools::{env::get_home_folder, question_yes_no};
use std::{fs, path::Path};

pub fn export_templates(export_file: String) {
    // Get all templates
    let templates = get_existing_templates();
    let home_folder = get_home_folder();

    let mut toml: Vec<String> = Vec::new();
    for template in templates {
        let mut template = process_template_to_struct(&template);

        // Replace home directory with ~ so it is portable
        template.path = template.path.replace(&home_folder, "~");

        toml.push(format!(
            "[{}]\nname = \"{}\"\npath = \"{}\"\ngit_path = \"{}\"\n\n",
            template.name, template.name, template.path, template.git_path
        ));
    }

    // Sort templates by name alphabetically
    toml.sort();

    // Covert to string
    // fs::write doesn't work with Vec<String>
    let toml: String = toml.join("");

    // Remove last newline
    // toml.pop();

    // Check if file already exists
    let export_file_path = Path::new(&export_file);
    if export_file_path.exists() {
        println!("File already exists");
        question_yes_no!("Do you want to overwrite it?");
    }

    // Write to file
    fs::write(&export_file, toml).unwrap();

    println!("Exported templates to \"{export_file}\"");
}
