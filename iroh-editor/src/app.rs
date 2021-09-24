use crate::{message::Message, pane_zone::PaneZone, panes::FieldWidget, theme::Theme};
use iced::{Element, Sandbox};
use iroh::{Kind, ObjectContainer};

/// State of our actual editor.
pub struct AppState<K: Kind, C: ObjectContainer<K>> {
    /// Currently selected object
    selected: Option<K::Key>,

    /// Volatile container for our objects
    container: C,

    /// Appearance settings
    theme: Theme,
}

impl<K: Kind, C: ObjectContainer<K>> AppState<K, C> {
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

    pub fn selected(&self) -> Option<&K> {
        self.selected.and_then(|x| self.container.get(x))
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
pub struct App<K: Kind, C: ObjectContainer<K>, F: FieldWidget<K>> {
    /// Stores state for splitting & moving around panes
    pane_zone: PaneZone<K, C, F>,

    /// Stores our actual application state
    app_state: AppState<K, C>,
}

impl<K: Kind, C: ObjectContainer<K>, F: 'static + FieldWidget<K>> Sandbox for App<K, C, F> {
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
            Message::PaneMessage(msg) => {
                self.pane_zone.update(&mut self.app_state, msg);
            }
            Message::Select(x) => self.app_state.select(Some(x)),
            Message::NewObject => {
                let k = self.app_state.container_mut().new();
                self.app_state.select(Some(k));
            }
            Message::Mutate(_) => todo!(),
        }
    }
}
