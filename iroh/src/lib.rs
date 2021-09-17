mod kinds;

pub use kinds::{KindsCollection, Kind, KindDesc, ConsKinds};

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectContainer {
    type Kinds: KindsCollection;
}
