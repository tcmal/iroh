//! For when you want to actually run the editor

use crate::{message::Message, pane_zone::PaneZone, theme::Theme, Kind, ObjectStore};
use iced::{Element, Sandbox};

/// State of our actual editor.
pub struct AppState<K: Kind, C: ObjectStore<K>> {
    /// Currently selected object
    selected: Option<K::Key>,

    /// Volatile container for our objects
    container: C,

    /// Appearance settings
    theme: Theme,
}

impl<K: Kind, C: ObjectStore<K>> AppState<K, C> {
    /// Get a reference to the app's theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Set the new selected object, if it exists. Otherwise, selection is cleared.
    pub fn select(&mut self, selected: Option<K::Key>) {
        self.selected = match selected {
            Some(x) => match self.container.exists(x) {
                true => Some(x),
                false => None,
            },
            None => None,
        }
    }

    /// Get a reference to the currently selected object
    pub fn selected(&self) -> Option<&K> {
        self.selected.and_then(|x| self.container.get(x))
    }

    /// Get a mutable reference to the currently selected object
    pub fn selected_mut(&mut self) -> Option<&mut K> {
        self.selected.and_then(move |x| self.container.get_mut(x))
    }

    /// Get the currently selected key
    pub fn selected_key(&self) -> Option<K::Key> {
        self.selected
    }

    pub fn is_selected(&self, key: K::Key) -> bool {
        match self.selected {
            Some(x) => x == key,
            None => false,
        }
    }

    /// Get a reference to the object container.
    pub fn container(&self) -> &C {
        &self.container
    }

    /// Get a mutable reference to the object container.
    pub fn container_mut(&mut self) -> &mut C {
        &mut self.container
    }
}

/// The main editor window
pub struct App<K: Kind, C: ObjectStore<K>> {
    /// Stores state for splitting & moving around panes
    pane_zone: PaneZone<K, C>,

    /// Stores our actual application state
    app_state: AppState<K, C>,
}

impl<K: Kind, C: ObjectStore<K>> Sandbox for App<K, C> {
    type Message = Message<K>;

    fn new() -> Self {
        let app_state = AppState {
            selected: None,
            theme: Theme::default(),
            container: C::empty(),
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
            Message::Nop => (),
            Message::PaneMessage(msg) => {
                self.pane_zone.update(&mut self.app_state, msg);
            }
            Message::Select(x) => self.app_state.select(Some(x)),
            Message::NewObject => {
                let k = self.app_state.container_mut().new();
                self.app_state.select(Some(k));
            }
            Message::Mutate(m) => {
                if let Some(o) = self.app_state.selected_mut() {
                    println!("{:?} {:?}", m, o);
                    m.apply(o);
                    println!("{:?}", o)
                }
            }
        }
    }
}
