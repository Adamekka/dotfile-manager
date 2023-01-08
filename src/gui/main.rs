#[path = "../lib.rs"]
mod lib;

use gtk::{
    gtk::{Box, Button, Orientation},
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,
};
use lib::set_folders;

const APP_ID: &str = "com.adamekka.dotfile-manager";

fn main() {
    println!("dman-gui found on path");
    set_folders();

    // Create new app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of 'app'
    app.connect_activate(build_ui);

    // Run app
    app.run();
}

fn build_ui(app: &Application) {
    // let button = Button::builder().label("press").build();

    // button.connect_clicked(move |button| button.set_label("clicked"));

    // Title bar
    let title_bar = Box::new(Orientation::Vertical, 0);
    title_bar.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Dotfile Manager by Adamekka", ""))
            .build(),
    );

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dotfile Manager by Adamekka")
        .default_height(480)
        .default_width(640)
        .content(&title_bar)
        // .content(&button)
        .build();

    // Present window
    window.present();
}
