use iced::{Element, Sandbox, Settings};

pub mod message;
pub mod panes;
mod theme;

use message::Message;
use panes::PaneZone;
use theme::Theme;

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
    App::run(Settings::default()).unwrap();
}
