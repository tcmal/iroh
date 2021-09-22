use iced::{scrollable, Align, Column, Row, Scrollable};
use iroh::Kind;

use crate::pane_zone::Paneable;

pub struct OutlinePane {
    scrollable_state: scrollable::State,
}

impl<K: Kind> Paneable<K> for OutlinePane {
    fn view<C>(
        &mut self,
        pane: iced::pane_grid::Pane,
        app_state: &crate::app::AppState<K, C>,
    ) -> iced::Element<crate::message::Message<K>> {
        let controls = Row::with_children(vec![]).align_items(Align::End).into();
        let list = Scrollable::new(&mut self.scrollable_state).into();

        Column::with_children(vec![controls, list]).into()
    }

    fn title(&self) -> String {
        "Outline".to_string()
    }
}

impl Default for OutlinePane {
    fn default() -> Self {
        Self {
            scrollable_state: scrollable::State::default(),
        }
    }
}
