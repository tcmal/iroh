//! Stores are used to keep track of what's in a file while the editor is open.
mod vec;

use crate::{Field, Kind};

pub use vec::VecContainer;

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectStore<K: 'static + Kind> {
    fn empty() -> Self;
    fn add(&mut self) -> &K::Key;

    type Items<'a>: Iterator<
        Item = (
            &'a K::Key,
            &'a K,
            &'a <<K as Kind>::Field as Field>::WorkingValues,
        ),
    >;
    fn items<'a>(&'a self) -> Self::Items<'a>;

    type Keys<'a>: Iterator<Item = &'a K::Key>;
    fn keys<'a>(&'a self) -> Self::Keys<'a>;

    type Values<'a>: Iterator<Item = &'a K>;
    fn values<'a>(&'a self) -> Self::Values<'a>;

    fn get(&self, key: &K::Key) -> Option<(&K, &<<K as Kind>::Field as Field>::WorkingValues)>;
    fn get_mut(
        &mut self,
        key: &K::Key,
    ) -> Option<(&mut K, &mut <<K as Kind>::Field as Field>::WorkingValues)>;

    fn exists(&self, key: &K::Key) -> bool;
    fn count(&self) -> usize;
}
