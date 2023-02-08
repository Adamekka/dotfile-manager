use crate::lib;

use lib::{get_existing_templates, process_template_to_struct, Template};
use owo_colors::OwoColorize;
use tabled::{
    color::Color,
    object::Segment,
    style::{BorderColored, RawStyle},
    Highlight, Style, Table,
};

impl Template {
    fn new(name: String, path: String, git_path: String) -> Self {
        Self {
            name,
            path,
            git_path,
        }
    }
}

/// Print listed templates
pub fn list_templates() {
    println!("Listing templates...");

    let templates = get_existing_templates();
    let mut data: Vec<Template> = Vec::new();

    for template_file in templates {
        let template = process_template_to_struct(&template_file);
        data.push(Template::new(
            template.name,
            template.path,
            template.git_path,
        ));
    }

    // If no templates found, push dummy data
    if data.is_empty() {
        data.push(Template::new(
            "No templates found".red().to_string(),
            "No templates found".red().to_string(),
            "No templates found".red().to_string(),
        ));
    }

    let table_style = RawStyle::from(Style::rounded()).colored();

    let color = Color::try_from(" ".red().to_string()).unwrap();

    let mut table = Table::from_iter(&data);
    table
        .with(table_style)
        .with(Highlight::colored(Segment::all(), BorderColored::default()))
        .with(color);

    println!("{table}");
}
