//! The inspector pane

use crate::{app::AppState, message::Message, pane_zone::Paneable, Kind, ObjectStore};
use iced::{pane_grid::Pane, Column, Element, Text};
use std::marker::PhantomData;

/// Shows the fields of the currently selected object.
pub struct InspectorPane<K: Kind, F: FieldWidget<K>>(F, PhantomData<K>);
impl<K: Kind, F: FieldWidget<K>> Default for InspectorPane<K, F> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}
impl<K: Kind, C: ObjectStore<K>, F: FieldWidget<K>> Paneable<K, C> for InspectorPane<K, F> {
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

pub trait FieldWidget<K: Kind>: Default {
    fn view(&mut self, val: &K) -> Vec<Element<Message<K>>>;
}

pub struct ConsFieldWidgets<A, B, K>(A, B, PhantomData<K>);
impl<K: Kind, A: FieldWidget<K>, B: FieldWidget<K>> Default for ConsFieldWidgets<A, B, K> {
    fn default() -> Self {
        Self(Default::default(), Default::default(), PhantomData)
    }
}
impl<K: Kind, A: FieldWidget<K>, B: FieldWidget<K>> FieldWidget<K> for ConsFieldWidgets<A, B, K> {
    fn view(&mut self, val: &K) -> Vec<Element<Message<K>>> {
        let mut v = self.0.view(val);
        v.extend(self.1.view(val));

        v
    }
}
