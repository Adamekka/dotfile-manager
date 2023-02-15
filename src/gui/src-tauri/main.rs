#![allow(dead_code)]

#[path = "../../lib.rs"]
mod lib;

use lib::get_templates_to_vec;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![reload_templates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Create a list of templates from the Templates folder
#[tauri::command]
fn reload_templates() -> Vec<[String; 3]> {
    let templates = get_templates_to_vec();
    let mut result: Vec<[String; 3]> = Vec::new();
    for template in templates {
        result.push([template.name, template.path, template.git_path]);
    }

    result
}
