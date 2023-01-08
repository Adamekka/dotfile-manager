#[path = "../lib.rs"]
mod lib;

use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings, Alignment};
use lib::set_folders;

struct Counter {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { count: (0) }
    }

    fn title(&self) -> String {
        String::from("Dotfile Manager by Adamekka")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.count).size(50),
            button("-").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    println!("dman-gui found on path");
    set_folders();

    Counter::run(Settings::default())
}
