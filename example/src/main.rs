use iced::{Sandbox, Settings};
use iroh::{containers::VecContainer, Field, Kind};
use iroh_editor::App;

pub struct RectWidthField;
impl Field for RectWidthField {
    fn new() -> Self {
        Self
    }
}

#[derive(Clone, Debug)]
pub struct Rect {
    id: usize,
    width: f32,
    height: f32,
}
impl Kind for Rect {
    type Key = usize;
    type Field = RectWidthField;

    fn key(&self) -> Self::Key {
        self.id
    }
}

// pub struct Circle {
//     radius: f32,
// }
// impl Kind for Circle {
//     fn kind_desc() -> KindDesc {
//         KindDesc {}
//     }
// }

fn main() {
    App::<Rect, VecContainer<_>>::run(Settings::default()).unwrap();
}
