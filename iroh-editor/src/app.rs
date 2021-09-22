use crate::{message::Message, pane_zone::PaneZone, theme::Theme};
use iced::{Element, Sandbox};
use iroh::{Kind, ObjectContainer};
use std::marker::PhantomData;

/// State of our actual editor.
pub struct AppState<'a, K: 'a + Kind, C: ObjectContainer<'a, K>> {
    /// Currently selected object
    selected: Option<K::Key>,

    /// Volatile container for our objects
    container: C,

    /// Appearance settings
    theme: Theme,
    _d: PhantomData<&'a C>,
}

impl<'a, K: Kind, C: ObjectContainer<'a, K>> AppState<'a, K, C> {
    /// Get a reference to the app's theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

/// The main editor window
pub struct App<'a, K: Kind, C: ObjectContainer<'a, K>> {
    /// Stores state for splitting & moving around panes
    pane_zone: PaneZone<'a, K, C>,

    /// Stores our actual application state
    app_state: AppState<'a, K, C>,
}

impl<'a, K: Kind, F: ObjectContainer<'a, K>> Sandbox for App<'a, K, F> {
    type Message = Message;

    fn new() -> Self {
        let app_state = AppState {
            selected: None,
            theme: Theme::default(),
            container: F::empty(),
            _d: PhantomData,
        };
        Self {
            pane_zone: PaneZone::new(&app_state),
            app_state,
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.pane_zone.view(&self.app_state)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PaneMessage(msg) => {
                self.pane_zone.update(&mut self.app_state, msg);
            }
            _ => todo!(),
        }
    }
}
