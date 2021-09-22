pub mod containers;
mod fields;
mod kinds;

pub use fields::{ConsFields, Field};
pub use kinds::{Key, Kind};

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectContainer<'a, K: Kind + 'a> {
    type AllIter: Iterator<Item = &'a K>;
    fn all(&'a self) -> Self::AllIter;
    fn empty() -> Self;
}
