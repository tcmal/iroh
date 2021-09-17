use iced::{Element, Sandbox, Settings};

pub mod message;
pub mod panes;

use message::Message;
use panes::PaneZone;

struct App {
    pane_zone: PaneZone,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let pane_zone = PaneZone::default();
        Self { pane_zone }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&mut self) -> Element<Message> {
        self.pane_zone.view()
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
    App::run(Settings::default()).unwrap();
}
