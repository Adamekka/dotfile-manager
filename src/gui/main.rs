#[path = "../lib.rs"]
mod lib;

use lib::get_existing_templates;
use lib::process_template_to_struct;
use lib::set_folders;
use lib::Template;
use relm4::adw;
use relm4::{
    gtk::{
        self,
        traits::{BoxExt, GtkWindowExt, OrientableExt},
    },
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
enum AppInput {}

struct AppModel {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = ();

    fn init(
        count: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = view_output!();
        let model = AppModel {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {}

    view!(adw::Window {
        set_title: Some("Dotfile Manager"),
        set_default_width: 640,
        set_default_height: 480,

        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            adw::HeaderBar {},

            // gtk::ListBox {
            // }
        }
    });
}

fn main() {
    println!("dman-gui found on path");
    set_folders();
    get_templates_to_vec();

    let app = RelmApp::new("dman-gui");
    app.run::<AppModel>(());
}

fn get_templates_to_vec() -> Vec<Template> {
    let templates = get_existing_templates();
    let mut templates_vec: Vec<Template> = Vec::new();

    for template in templates {
        let template = process_template_to_struct(&template);
        templates_vec.push(template);
    }

    dbg!(&templates_vec);

    templates_vec
}
