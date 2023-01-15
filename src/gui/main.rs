#[path = "../lib.rs"]
mod lib;

use lib::set_folders;
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

    view!(gtk::Window {
        set_title: Some("Dotfile Manager"),
        set_default_width: 300,
        set_default_height: 100,

        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Label {
                set_label: &format!("Dotfile Manager"),
                set_margin_all: 5,
            }
        }
    });
}

fn main() {
    println!("dman-gui found on path");
    set_folders();

    let app = RelmApp::new("dman-gui");
    app.run::<AppModel>(());
}
