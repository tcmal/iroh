#![feature(generic_associated_types)]

pub mod app;
pub mod containers;
mod kinds;
pub mod message;
pub mod mutation;
pub mod pane_zone;
pub mod panes;
pub mod theme;

pub use app::App;
pub use iced::{Sandbox, Settings};
pub use kinds::{Key, Kind};

/// A container for objects of differing kinds. Usually, this will be your filetype.
pub trait ObjectContainer<K: 'static + Kind> {
    fn empty() -> Self;
    fn new(&mut self) -> K::Key;

    type AllIter<'a>: Iterator<Item = &'a K>;
    fn all<'a>(&'a self) -> Self::AllIter<'a>;
    fn exists(&self, key: K::Key) -> bool;
    fn get(&self, key: K::Key) -> Option<&K>;
    fn count(&self) -> usize;
}
