//! Traits and structs related to the representation of kinds.
use crate::fields::Field;
use std::fmt::Debug;

pub trait Kind: 'static + Clone + Debug {
    type Key: Key;
    type Field: Field;

    fn key(&self) -> Self::Key;
}

pub trait Key: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
impl<T> Key for T where T: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
