#[path = "../lib.rs"]
mod lib;

use gtk::{
    glib::clone,
    glib::clone::{Downgrade, Upgrade},
    gtk::{Box, Button, ListBox, Orientation, SelectionMode},
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
    ActionRow, Application, ApplicationWindow, HeaderBar, WindowTitle,
};
use lib::set_folders;
use std::{cell::Cell, rc::Rc};

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
    let first_row = ActionRow::builder()
        .activatable(true)
        .title("first row")
        .build();

    let templates_list = ListBox::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();
    templates_list.append(&first_row);

    // let button_increase = Button::builder()
    //     .label("increase")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_start(256)
    //     .margin_end(256)
    //     .build();

    // let button_decrease = Button::builder()
    //     .label("decrease")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_start(256)
    //     .margin_end(256)
    //     .build();

    // let count = Rc::new(Cell::new(0));

    // button_increase.connect_clicked(clone!(@strong count, @strong button_decrease =>
    //     move |_| {
    //         count.set(count.get() + 1);
    //         button_decrease.set_label(&count.get().to_string());
    // }));
    // button_decrease.connect_clicked(clone!(@strong button_increase =>
    //     move |_| {
    //         count.set(count.get() - 1);
    //         button_increase.set_label(&count.get().to_string());
    // }));

    // Title bar
    let title_bar = Box::new(Orientation::Vertical, 0);
    title_bar.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Dotfile Manager", "by Adamekka"))
            .build(),
    );

    let gtk_box = Box::builder().orientation(Orientation::Vertical).build();
    gtk_box.append(&title_bar);
    // gtk_box.append(&button_increase);
    // gtk_box.append(&button_decrease);
    gtk_box.append(&templates_list);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dotfile Manager by Adamekka")
        .default_height(480)
        .default_width(640)
        .content(&gtk_box)
        .build();

    // Present window
    window.present();
}
