mod kinds;

pub use kinds::{ConsKinds, Kind, KindDesc, KindsCollection};

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectContainer {
    type Kinds: KindsCollection;
}
