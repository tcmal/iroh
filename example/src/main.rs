use iced::{Element, Text};
use iroh::{containers::VecContainer, Kind};
use iroh_editor::{message::Message, panes::FieldWidget, *};

/// Example kind
#[derive(Clone, Debug)]
pub struct Rect {
    id: usize,
    width: f32,
    height: f32,
}

impl Kind for Rect {
    type Key = usize;

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

struct RectFieldWidget;
impl Default for RectFieldWidget {
    fn default() -> Self {
        Self {}
    }
}
impl FieldWidget<Rect> for RectFieldWidget {
    fn view(&mut self, val: &Rect) -> Vec<Element<Message<Rect>>> {
        vec![Text::new(format!("{:?}", val)).into()]
    }
}

fn main() {
    App::<Rect, VecContainer<_>, RectFieldWidget>::run(Settings::default()).unwrap();
}
