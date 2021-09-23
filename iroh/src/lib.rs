#![feature(generic_associated_types)]

pub mod containers;
// TODO mod fields;
mod kinds;

pub use kinds::{Key, Kind};

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectContainer<K: 'static + Kind> {
    fn empty() -> Self;
    fn new(&mut self) -> K::Key;

    type AllIter<'a>: Iterator<Item = &'a K>;
    fn all<'a>(&'a self) -> Self::AllIter<'a>;
    fn exists(&self, key: K::Key) -> bool;
    fn count(&self) -> usize;
}
