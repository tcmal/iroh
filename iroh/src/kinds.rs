//! Traits and structs related to the representation of kinds.
use crate::fields::Field;
use std::fmt::Debug;

pub trait Kind: 'static + Clone + Debug {
    type Key: Key;
    type Field: Field;

    fn key(&self) -> Self::Key;
    fn default_with_key(key: Self::Key) -> Self;
}

pub trait Key: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
impl<T> Key for T where T: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
