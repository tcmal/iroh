//! The inspector pane

use crate::{app::AppState, message::Message, pane_zone::Paneable, Field, Kind, ObjectStore};
use iced::{pane_grid::Pane, Column, Element, Text};

/// Shows the fields of the currently selected object.
pub struct InspectorPane<F: Field>(F);
impl<F: Field> Default for InspectorPane<F> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<K: Kind, C: ObjectStore<K>, F: Field<Kind = K>> Paneable<K, C> for InspectorPane<F> {
    fn view(&mut self, _pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K>> {
        if let Some(val) = app_state.selected() {
            let mut col = Column::new();

            for e in self.0.view(val) {
                col = col.push(e);
            }

            col.into()
        } else {
            Text::new("No object selected.").into()
        }
    }

    fn title(&self) -> String {
        "Inspector".to_string()
    }
}
