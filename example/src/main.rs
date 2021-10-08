use iroh::{fields::TextInputField, kinds::ConsFields, stores::VecContainer, Kind, *};

#[macro_use]
extern crate iroh_codegen;

/// Example kind
#[derive(Clone, Debug, Lens)]
pub struct Rect {
    width: f32,
    height: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}
impl Kind for Rect {
    type Key = RectId;
    type Field = ConsFields<
        TextInputField<"Width", RectWidthLens>,
        TextInputField<"Height", RectHeightLens>,
    >;
}

/// The key for our example kind
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RectId(pub usize);

impl Key for RectId {
    fn first() -> Self {
        RectId(0)
    }

    fn next(last: &Self) -> Self {
        RectId(last.0 + 1)
    }
}

fn main() {
    App::<Rect, VecContainer<_>>::run(Settings::default()).unwrap();
}
