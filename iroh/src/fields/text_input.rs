use crate::{
    app::AppState,
    lens::{Lens, RootLens},
    message::Message,
    mutation::{LensSet, NopMutator},
    Field, Kind, ObjectStore,
};
use iced::{text_input, Align, Element, Row, Text, TextInput};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

/// A field which the user sets a value for using a text input.
/// Uses a lens to get/set the right field on the struct.
#[derive(Debug, Clone)]
pub struct TextInputField<const N: &'static str, L> {
    string_value: String,
    input_state: text_input::State,
    _d: PhantomData<L>,
}
impl<const N: &'static str, L: 'static + Lens> Field for TextInputField<N, L>
where
    L::Source: Kind,
    L::Target: 'static + FromTextInput + Debug + Clone + Send,
{
    type Kind = L::Source;
    type WorkingValues = Option<String>;

    fn view<C: ObjectStore<Self::Kind>>(
        &mut self,
        _key: &<L::Source as Kind>::Key,
        val: &L::Source,
        app_state: &AppState<Self::Kind, C>,
        working: &Option<String>,
    ) -> Vec<Element<Message<L::Source, Self::WorkingValues>>> {
        let _out_of_sync = if let Some(w) = working.clone() {
            self.string_value = w;

            true
        } else {
            self.string_value = format!("{}", L::get(val));

            false
        };

        // TODO: Style based on `_out_of_sync`

        vec![Row::with_children(vec![
            Text::new(N).color(app_state.theme().text_primary()).into(),
            TextInput::new(&mut self.input_state, N, &self.string_value, |new| {
                if let Some(v) = L::Target::from_input(&new) {
                    return Message::Mutate(
                        Box::new(LensSet::<L>::new(v)),
                        Box::new(LensSet::<RootLens<_>>::new(None)),
                    );
                } else {
                    Message::Mutate(
                        Box::new(NopMutator),
                        Box::new(LensSet::<RootLens<_>>::new(Some(new))),
                    )
                }
            })
            .style(app_state.theme().text_input())
            .padding(5)
            .into(),
        ])
        .spacing(4)
        .padding(8)
        .align_items(Align::Center)
        .into()]
    }
}

/// A type that can be converted to from a text input.
/// We don't use FromStr because we want full control over what valid input is.
/// For instance, when parsing floats we fail when there's a trailing . because otherwise
/// they are immediately erased.
pub trait FromTextInput: Sized + Display {
    fn from_input(s: &str) -> Option<Self>;
}

impl<const N: &'static str, L> Default for TextInputField<N, L> {
    fn default() -> Self {
        Self {
            input_state: text_input::State::default(),
            string_value: "".to_string(),
            _d: PhantomData,
        }
    }
}

/// Implementation for anything that implements FromStr
/// This also checks that the [`std::fmt::Display`] representation is the same as what the user inputted.
impl<T: Display + FromStr> FromTextInput for T {
    fn from_input(s: &str) -> Option<Self> {
        if let Ok(x) = <Self as FromStr>::from_str(s) {
            if format!("{}", x) == s {
                return Some(x);
            }
        }

        None
    }
}
