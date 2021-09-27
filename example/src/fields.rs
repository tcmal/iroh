//! Fields for our `Rect` type.

use crate::{lens::*, Rect, RectId};
use iced::{text_input, Element, Row, Text, TextInput};
use iroh::{
    lens::RootLens,
    message::Message,
    mutation::{LensSet, NopMutator},
    Field,
};

#[derive(Debug, Clone)]
pub struct RectWidthField {
    string_value: String,
    input_state: text_input::State,
}
impl Default for RectWidthField {
    fn default() -> Self {
        Self {
            input_state: text_input::State::default(),
            string_value: "".to_string(),
        }
    }
}
impl Field for RectWidthField {
    type Kind = Rect;
    type WorkingValues = Option<String>;

    fn view(
        &mut self,
        _key: &RectId,
        val: &Rect,
        working: &Option<String>,
    ) -> Vec<Element<Message<Rect, Self::WorkingValues>>> {
        let _out_of_sync = if let Some(w) = working.clone() {
            self.string_value = w;

            true
        } else {
            self.string_value = format!("{}", val.width);

            false
        };

        // TODO: Style based on `_out_of_sync`

        vec![Row::with_children(vec![
            Text::new("Width").into(),
            TextInput::new(&mut self.input_state, "Width", &self.string_value, |new| {
                if let Ok(v) = new.parse() {
                    if format!("{}", v) == new {
                        return Message::Mutate(
                            Box::new(LensSet::<RectWidthLens>::new(v)),
                            Box::new(LensSet::<RootLens<_>>::new(None)),
                        );
                    }
                }

                Message::Mutate(
                    Box::new(NopMutator),
                    Box::new(LensSet::<RootLens<_>>::new(Some(new))),
                )
            })
            .into(),
        ])
        .into()]
    }
}

// This is almost entirely copy-pasted from above.

#[derive(Debug, Clone)]
pub struct RectHeightField {
    string_value: String,
    input_state: text_input::State,
}
impl Default for RectHeightField {
    fn default() -> Self {
        Self {
            input_state: text_input::State::default(),
            string_value: "".to_string(),
        }
    }
}
impl Field for RectHeightField {
    type Kind = Rect;
    type WorkingValues = Option<String>;

    fn view(
        &mut self,
        _key: &RectId,
        val: &Rect,
        working: &Option<String>,
    ) -> Vec<Element<Message<Rect, Self::WorkingValues>>> {
        let _out_of_sync = if let Some(w) = working.clone() {
            self.string_value = w;

            true
        } else {
            self.string_value = format!("{}", val.height);

            false
        };

        // TODO: Style based on `_out_of_sync`

        vec![Row::with_children(vec![
            Text::new("Height").into(),
            TextInput::new(&mut self.input_state, "Height", &self.string_value, |new| {
                if let Ok(v) = new.parse() {
                    if format!("{}", v) == new {
                        return Message::Mutate(
                            Box::new(LensSet::<RectHeightLens>::new(v)),
                            Box::new(LensSet::<RootLens<_>>::new(None)),
                        );
                    }
                }

                Message::Mutate(
                    Box::new(NopMutator),
                    Box::new(LensSet::<RootLens<_>>::new(Some(new))),
                )
            })
            .into(),
        ])
        .into()]
    }
}
