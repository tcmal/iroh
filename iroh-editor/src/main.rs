use iced::{Element, Sandbox, Settings};

pub mod ffi;
pub mod message;
pub mod panes;
mod theme;

use ffi::Schema;
use message::Message;
use panes::PaneZone;
use theme::Theme;

/// The main editor window
struct App {
    pane_zone: PaneZone,
    theme: Theme,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            pane_zone: PaneZone::default(),
            theme: Theme::default(),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&mut self) -> Element<Message> {
        self.pane_zone.view(&self.theme)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PaneMessage(msg) => {
                self.pane_zone.update(msg);
            }
        }
    }
}

fn main() {
    let path = &std::env::args().collect::<Vec<_>>()[1];
    let schema = Schema::new(path).unwrap();
    println!("{:?}", schema);

    App::run(Settings::default()).unwrap();
}
