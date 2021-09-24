//! Stores are used to keep track of what's in a file while the editor is open.
mod vec;

use crate::Kind;

pub use vec::VecContainer;

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectStore<K: 'static + Kind> {
    fn empty() -> Self;
    fn new(&mut self) -> &K::Key;

    type AllIter<'a>: Iterator<Item = &'a K>;
    fn all<'a>(&'a self) -> Self::AllIter<'a>;

    fn exists(&self, key: &K::Key) -> bool;
    fn get(&self, key: &K::Key) -> Option<&K>;
    fn get_mut(&mut self, key: &K::Key) -> Option<&mut K>;
    fn count(&self) -> usize;
}
