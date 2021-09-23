use crate::{app::AppState, message::Message, pane_zone::Paneable};
use iced::{
    button, pane_grid::Pane, scrollable, Align, Button, Column, Element, Length, Row, Scrollable,
    Text,
};
use iroh::{Kind, ObjectContainer};

pub struct OutlinePane {
    scrollable_state: scrollable::State,
    states: Vec<button::State>,
}

impl<K: Kind, C: ObjectContainer<K>> Paneable<K, C> for OutlinePane {
    fn view(&mut self, _pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K::Key>> {
        let controls = Row::with_children(vec![]).align_items(Align::End);
        let mut list = Scrollable::new(&mut self.scrollable_state);

        while self.states.len() < app_state.container().count() {
            self.states.push(button::State::default());
        }

        for (v, s) in app_state.container().all().zip(self.states.iter_mut()) {
            list = list.push(
                Button::new(s, Text::new(format!("{:?}", v.key())))
                    .on_press(Message::Select(v.key()))
                    .style(app_state.theme().button_subtle())
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
            states: vec![],
        }
    }
}
