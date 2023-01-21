use crate::lib;

use lib::get_existing_templates;
use lib::process_template_to_struct;
use lib::Template;
use owo_colors::OwoColorize;
use tabled::color::Color;
use tabled::object::Rows;
use tabled::object::Segment;
use tabled::style::BorderColored;
use tabled::style::RawStyle;
use tabled::style::Symbol;
use tabled::Highlight;
use tabled::ModifyObject;
use tabled::Style;
use tabled::Table;

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

    let table_style = RawStyle::from(Style::rounded()).colored();

    let color = Color::try_from(" ".red().to_string()).unwrap();

    let mut table = Table::from_iter(&data);
    table
        .with(table_style)
        .with(Highlight::colored(Segment::all(), BorderColored::default()))
        .with(color);

    println!("{}", table);
}
