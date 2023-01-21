use crate::lib;

use lib::get_existing_templates;
use lib::process_template_to_struct;
use lib::Template;
use owo_colors::OwoColorize;
use tabled::object::Segment;
use tabled::style::BorderColored;
use tabled::style::RawStyle;
use tabled::style::Symbol;
use tabled::Highlight;
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

    let mut table_style = RawStyle::from(Style::extended()).colored();
    table_style.set_horizontal(Some(Symbol::ansi("═".magenta().to_string()).unwrap()));
    table_style.set_vertical(Some(Symbol::ansi("║".cyan().to_string()).unwrap()));

    let mut table = Table::from_iter(&data);
    table.with(table_style).with(Highlight::colored(
        Segment::all(),
        BorderColored::default()
            .top(Symbol::ansi("═".red().to_string()).unwrap())
            .bottom(Symbol::ansi("═".red().to_string()).unwrap())
            .left(Symbol::ansi("║".red().to_string()).unwrap())
            .right(Symbol::ansi("║".red().to_string()).unwrap())
            .top_left_corner(Symbol::ansi("╔".red().to_string()).unwrap())
            .top_right_corner(Symbol::ansi("╗".red().to_string()).unwrap())
            .bottom_left_corner(Symbol::ansi("╚".red().to_string()).unwrap())
            .bottom_right_corner(Symbol::ansi("╝".red().to_string()).unwrap()),
    ));

    println!("{}", table);
}
