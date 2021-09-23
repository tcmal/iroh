use iroh::{containers::VecContainer, Kind};
use iroh_editor::*;

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

fn main() {
    App::<Rect, VecContainer<_>>::run(Settings::default()).unwrap();
}
