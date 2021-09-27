pub mod inspector;
pub mod outline;

pub use crate::pane_zone::Paneable;
pub use outline::OutlinePane;

use crate::{
    app::AppState,
    message::{Message, NewPane, PaneMessage},
    Kind, ObjectStore,
};
use iced::{button, pane_grid::Pane, Button, Column, Element, Text};

/// An empty pane, which provides buttons to swap it out for any other pane.
pub struct EmptyPane {
    outline_state: button::State,
    inspector_state: button::State,
}
impl EmptyPane {
    /// Create a new empty pane
    pub fn new() -> Self {
        Self {
            outline_state: button::State::default(),
            inspector_state: button::State::default(),
        }
    }
}
impl Default for EmptyPane {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Kind, C: ObjectStore<K>> Paneable<K, C> for EmptyPane {
    fn view(&mut self, pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K>> {
        Column::with_children(vec![
            // Outline
            Button::new(&mut self.outline_state, Text::new("Outline"))
                .style(app_state.theme().button_primary())
                .on_press(PaneMessage::Set(pane, NewPane::Outline).into())
                .into(),
            // Inspector
            Button::new(&mut self.inspector_state, Text::new("Inspector"))
                .style(app_state.theme().button_primary())
                .on_press(PaneMessage::Set(pane, NewPane::Inspector).into())
                .into(),
        ])
        .padding(10)
        .spacing(10)
        .into()
    }

    fn title(&self) -> String {
        "Empty".into()
    }
}
