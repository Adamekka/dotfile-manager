#[path = "../lib.rs"]
mod lib;

use adw::prelude::*;
use gtk::prelude::*;
use lib::{get_existing_templates, process_template_to_struct, set_folders, Template};
use relm4::{
    adw,
    adw::prelude::*,
    factory::FactoryVecDeque,
    gtk::prelude::*,
    gtk::{
        self,
        traits::{BoxExt, GtkWindowExt, OrientableExt},
    },
    prelude::*,
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

// #[relm4::factory(pub)]
// impl FactoryComponent for Template {
//     type ParentWidget = gtk::ListBox;
//     type ParentInput = ();
//     type CommandOutput = ();
//     type Input = ();
//     type Output = ();
//     type Init = Template;

//     fn init_model(template: Self::Init, index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
//         Self {
//             name: template.name,
//             path: template.path,
//             git_path: template.git_path,
//         }
//     }

//     view! {
//         gtk::Box {
//             set_orientation: gtk::Orientation::Horizontal,

//             #[name(label)]
//             gtk::Label {
//                 set_label: &self.name,
//                 set_hexpand: true,
//                 set_halign: gtk::Align::Start,
//                 set_margin_all: 12,
//             }
//         }
//     }
// }

// #[derive(Debug)]
// pub enum AppMsg {}

struct AppModel {
    // templates: FactoryVecDeque<Template>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = ();
    type Output = ();
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            // templates: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
        };

        // let templates_list_box = model.templates.widget();
        let templates_list_box = make_list_box(get_templates_to_vec());
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {}

    view!(
        main_window = adw::Window {
            set_title: Some("Dotfile Manager"),
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                // gtk::Entry {
                //     connect_activate[sender] => move |entry| {
                //         let buffer = entry.buffer();
                //         sender.input(App)
                //     }
                // }

                adw::HeaderBar {},

                gtk::Button {
                    set_label: "Reload Templates",
                    set_margin_all: 5,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                },

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 240,
                    set_vexpand: true,

                    #[local_ref]
                    templates_list_box -> gtk::ListBox {}
                }
            }
        }
    );
}

fn main() {
    println!("dman-gui found on path");
    set_folders();
    get_templates_to_vec();

    let app = RelmApp::new("dman-gui");
    app.run::<AppModel>(());
}

/// Read templates from filesystem and put them to Vector
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

fn make_list_box(templates: Vec<Template>) -> gtk::ListBox {
    let list_box = relm4::gtk::ListBox::new();
    for template in templates {
        let label = relm4::gtk::Label::new(Some(&template.name));
        list_box.append(&label);
    }
    list_box
}
