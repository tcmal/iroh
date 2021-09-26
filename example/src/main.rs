use iced::{text_input, Element, Row, Text, TextInput};
use iroh::{
    message::Message,
    mutation::{Lens, LensSet, NopMutator},
    stores::VecContainer,
    Kind, *,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RectId(pub usize);

/// Example kind
#[derive(Clone, Debug)]
pub struct Rect {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone)]
struct RectWidthLens;
impl Lens for RectWidthLens {
    type Source = Rect;
    type Target = f32;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.width
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.width
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectWorkingValues {
    width: Option<String>,
    height: Option<String>,
}

impl Default for RectWorkingValues {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

#[derive(Debug, Clone)]
struct RectWorkingWidthLens;
impl Lens for RectWorkingWidthLens {
    type Source = RectWorkingValues;
    type Target = Option<String>;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.width
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.width
    }
}

impl Kind for Rect {
    type Key = RectId;
    type Field = RectWidthField;
    type WorkingValues = RectWorkingValues;
}

impl Key for RectId {
    fn first() -> Self {
        RectId(0)
    }

    fn next(last: &Self) -> Self {
        RectId(last.0 + 1)
    }
}

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

    fn view(
        &mut self,
        _key: &RectId,
        val: &Rect,
        working: &RectWorkingValues,
    ) -> Vec<Element<Message<Rect>>> {
        let _out_of_sync = if let Some(w) = working.width.clone() {
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
                            Box::new(LensSet::<RectWorkingWidthLens>::new(None)),
                        );
                    }
                }

                Message::Mutate(
                    Box::new(NopMutator),
                    Box::new(LensSet::<RectWorkingWidthLens>::new(Some(new))),
                )
            })
            .into(),
        ])
        .into()]
    }
}

fn main() {
    App::<Rect, VecContainer<_>>::run(Settings::default()).unwrap();
}
