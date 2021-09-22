// pub mod outline;

// pub use outline::OutlinePane;

use crate::{
    app::AppState,
    message::{Message, NewPane, PaneMessage},
    pane_zone::Paneable,
};
use iced::{button, pane_grid::Pane, Button, Column, Element, Text};
use iroh::{Kind, ObjectContainer};

/// An empty pane, which provides buttons to swap it out for any other pane.
pub struct EmptyPane {
    outline_state: button::State,
}
impl EmptyPane {
    pub fn new<'a, K: Kind, C: ObjectContainer<'a, K>>(_app_state: &AppState<'a, K, C>) -> Self {
        Self {
            outline_state: button::State::default(),
        }
    }
}
impl<'a, K: 'a + Kind, C: ObjectContainer<'a, K>> Paneable<'a, K, C> for EmptyPane {
    fn view(&mut self, pane: Pane, app_state: &AppState<'a, K, C>) -> Element<Message> {
        let col = Column::new().padding(10).spacing(10).push(
            Button::new(&mut self.outline_state, Text::new("Outline"))
                .style(app_state.theme().button_primary())
                .on_press(PaneMessage::Set(pane, NewPane::Outline).into()),
        );
        col.into()
    }

    fn title(&self) -> String {
        "Hello, World!".into()
    }
}
