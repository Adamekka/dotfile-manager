use crate::args::remove::remove_template;
use crate::lib;
use lib::{get_existing_templates, process_template_to_struct, Template};
use mytools::{question_yes_no, warn};
use owo_colors::OwoColorize;
use std::path::Path;
use tabled::{
    color::Color,
    object::{Rows, Segment},
    style::{BorderColored, RawStyle},
    Highlight, Modify, Style, Table, Width,
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
    let mut non_existing_templates: Vec<Template> = Vec::new();

    for template_file in templates {
        let template = process_template_to_struct(&template_file);
        data.push(Template::new(
            template.name.clone(),
            template.path.clone(),
            template.git_path.clone(),
        ));

        // Check if template is in filesystem
        let template_path = Path::new(&template.path);
        if !template_path.exists() {
            non_existing_templates.push(Template::new(
                template.name,
                template.path,
                template.git_path,
            ));
        }
    }

    // If no templates found, push dummy data
    if data.is_empty() {
        data.push(Template::new(
            "No templates found".red().to_string(),
            "No templates found".red().to_string(),
            "No templates found".red().to_string(),
        ));
    }

    // Sort templates by name alphabetically
    data.sort_by(|a, b| a.name.cmp(&b.name));

    let table_style = RawStyle::from(Style::rounded()).colored();

    let color = Color::try_from(" ".red().to_string()).unwrap();

    let mut table = Table::from_iter(&data);
    table
        .with(table_style)
        .with(Highlight::colored(Segment::all(), BorderColored::default()))
        .with(color);

    // Get terminal size
    let term_size = termion::terminal_size().expect("Failed to get terminal size");
    // Get table width
    let table_width = table.total_width();

    // If table is bigger than terminal, truncate words in it
    if table_width > term_size.0 as usize {
        table.with(
            Modify::new(Rows::new(1..))
                .with(Width::truncate(term_size.0 as usize / 3).suffix("..")),
        );
        warn!("Table is too big for terminal, so it's resized");
    }

    // Print table
    println!("{table}");

    if !non_existing_templates.is_empty() {
        warn!("Some templates are not in filesystem: {non_existing_templates:?}");
        question_yes_no!("Do you want to remove templates, that aren't in filesystem?");
        for template in non_existing_templates {
            remove_template(
                Some(template.name),
                Some(template.path),
                Some(template.git_path),
            )
        }
    }
}
