//! For when you want to actually run the editor

use crate::{message::Message, pane_zone::PaneZone, theme::Theme, Field, Kind, ObjectStore};
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
            Some(x) => match self.container.exists(&x) {
                true => Some(x),
                false => None,
            },
            None => None,
        }
    }

    /// Get a reference to the currently selected object and its working values
    pub fn selected(&self) -> Option<(&K::Key, &K, &<<K as Kind>::Field as Field>::WorkingValues)> {
        self.selected
            .as_ref()
            .and_then(|x| self.container.get(x).map(|(v, w)| (x, v, w)))
    }

    /// Get a mutable reference to the currently selected object and its working values
    pub fn selected_mut(
        &mut self,
    ) -> Option<(&mut K, &mut <<K as Kind>::Field as Field>::WorkingValues)> {
        match self.selected.as_ref() {
            Some(k) => self.container.get_mut(k),
            None => None,
        }
    }

    /// Get the currently selected key
    pub fn selected_key(&self) -> Option<&K::Key> {
        self.selected.as_ref()
    }

    pub fn is_selected(&self, key: &K::Key) -> bool {
        match self.selected.as_ref() {
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

    /// Create a new object, and select it
    pub fn new(&mut self) {
        let k = self.container.add().clone();
        self.select(Some(k));
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
            pane_zone: PaneZone::new(),
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
                self.pane_zone.update(msg);
            }
            Message::Select(x) => self.app_state.select(Some(x)),
            Message::NewObject => self.app_state.new(),
            Message::Mutate(vm, wm) => {
                if let Some((v, w)) = self.app_state.selected_mut() {
                    vm.apply(v);
                    wm.apply(w);
                }
            }
        }
    }
}
