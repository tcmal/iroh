use crate::{ffi::Schema, message::Message, panes::PaneZone, theme::Theme};
use iced::{Element, Sandbox};

pub struct AppState {
    theme: Theme,
    schema: Schema,
}

impl AppState {
    /// Get a reference to the app's theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Get a reference to the app state's schema.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

/// The main editor window
pub struct App {
    pane_zone: PaneZone,
    app_state: AppState,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let path = &std::env::args().collect::<Vec<_>>()[1];
        let schema = Schema::new(path).unwrap();
        let app_state = AppState {
            theme: Theme::default(),
            schema,
        };
        Self {
            pane_zone: PaneZone::new(&app_state),
            app_state,
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&mut self) -> Element<Message> {
        self.pane_zone.view(&self.app_state)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PaneMessage(msg) => {
                self.pane_zone.update(&mut self.app_state, msg);
            }
        }
    }
}
