pub mod inspector;
pub mod outline;
pub use inspector::FieldWidget;
pub use outline::OutlinePane;

use crate::{
    app::AppState,
    message::{Message, NewPane, PaneMessage},
    pane_zone::Paneable,
    Kind, ObjectContainer,
};
use iced::{button, pane_grid::Pane, Button, Column, Element, Text};

/// An empty pane, which provides buttons to swap it out for any other pane.
pub struct EmptyPane {
    outline_state: button::State,
    inspector_state: button::State,
}
impl EmptyPane {
    pub fn new<K: Kind, C: ObjectContainer<K>>(_app_state: &AppState<K, C>) -> Self {
        Self {
            outline_state: button::State::default(),
            inspector_state: button::State::default(),
        }
    }
}
impl<K: Kind, C: ObjectContainer<K>> Paneable<K, C> for EmptyPane {
    fn view(&mut self, pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K>> {
        Column::with_children(vec![
            Button::new(&mut self.outline_state, Text::new("Outline"))
                .style(app_state.theme().button_primary())
                .on_press(PaneMessage::Set(pane, NewPane::Outline).into())
                .into(),
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
        "Hello, World!".into()
    }
}