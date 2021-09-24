use iced::{text_input, Element, Row, Text, TextInput};
use iroh::{
    message::Message,
    mutation::{Lens, LensSet},
    stores::VecContainer,
    Kind, *,
};

/// Example kind
#[derive(Clone, Debug)]
pub struct Rect {
    id: usize,
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

impl Kind for Rect {
    type Key = usize;
    type Field = RectWidthField;

    fn key(&self) -> Self::Key {
        self.id
    }

    fn default_with_key(key: Self::Key) -> Self {
        Self {
            id: key,
            width: 1.0,
            height: 1.0,
        }
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

    fn view(&mut self, val: &Rect) -> Vec<Element<Message<Rect>>> {
        self.string_value = format!("{}", val.width);

        vec![Row::with_children(vec![
            Text::new("Width").into(),
            TextInput::new(&mut self.input_state, "Width", &self.string_value, |new| {
                if let Ok(v) = new.parse() {
                    Message::Mutate(Box::new(LensSet::<RectWidthLens>::new(v)))
                } else {
                    Message::Nop
                }
            })
            .into(),
        ])
        .into()]
    }
}

fn main() {
    App::<Rect, VecContainer<_>>::run(Settings::default()).unwrap();
}
