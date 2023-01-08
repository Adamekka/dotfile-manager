#[path = "../lib.rs"]
mod lib;

use gtk::{prelude::{ApplicationExtManual, GtkWindowExt, ApplicationExt}, Application, ApplicationWindow};
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
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dotfile Manager by Adamekka")
        .build();

    // Present window
    window.present();
}
