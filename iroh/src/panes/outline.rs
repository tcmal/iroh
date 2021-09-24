//! The outline pane

use crate::{app::AppState, message::Message, pane_zone::Paneable, Kind, ObjectStore};
use iced::{
    button, pane_grid::Pane, scrollable, Align, Button, Column, Element, Length, Row, Scrollable,
    Text,
};

/// Shows a list of all objects in the store for selection.
pub struct OutlinePane {
    scrollable_state: scrollable::State,
    new_state: button::State,
    states: Vec<button::State>,
}

impl<K: Kind, C: ObjectStore<K>> Paneable<K, C> for OutlinePane {
    fn view(&mut self, _pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K>> {
        let controls = Row::with_children(vec![Button::new(&mut self.new_state, Text::new("+"))
            .on_press(Message::NewObject)
            .style(app_state.theme().button_primary())
            .into()])
        .align_items(Align::End);
        let mut list = Scrollable::new(&mut self.scrollable_state);

        while self.states.len() < app_state.container().count() {
            self.states.push(button::State::default());
        }

        for (v, s) in app_state.container().all().zip(self.states.iter_mut()) {
            let selected = app_state.is_selected(v.key());
            list = list.push(
                Button::new(s, Text::new(format!("{:?}", v.key())))
                    .on_press(Message::Select(v.key()))
                    .style(if selected {
                        app_state.theme().button_primary()
                    } else {
                        app_state.theme().button_subtle()
                    })
                    .width(Length::Fill),
            );
        }

        Column::with_children(vec![controls.into(), list.into()]).into()
    }

    fn title(&self) -> String {
        "Outline".to_string()
    }
}

impl Default for OutlinePane {
    fn default() -> Self {
        Self {
            scrollable_state: scrollable::State::default(),
            new_state: button::State::default(),
            states: vec![],
        }
    }
}
