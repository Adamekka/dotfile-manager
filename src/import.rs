use std::path::Path;

/// Import templates from a file
pub fn import_templates(file_path: String) {
    let file_path = Path::new(&file_path);

    // check if file exists
    if !file_path.exists() {
        panic!("File does not exist");
    }

    // check if file is a toml file
    match file_path.extension() {
        Some(extension) => match extension.to_str() {
            Some(extension) => {
                if extension != "toml" {
                    panic!("File is not a toml file");
                }
            }
            None => {
                panic!("File is not a toml file");
            }
        },
        None => {
            panic!("File is not a toml file");
        }
    }

    // check if file is a valid toml file
    // check if file contains valid template(s)
    // check if template(s) already exists
    // import template(s)

    println!("Importing templates...");
}
