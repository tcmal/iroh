//! Traits and structs related to the representation of kinds.
use crate::fields::Field;
use std::fmt::Debug;

pub trait Kind: Clone + Debug {
    type Key: Key;
    type Field: Field;

    fn key(&self) -> Self::Key;
}

pub trait Key: Copy + Clone + Debug + Send + Sync {}
impl<T> Key for T where T: Copy + Clone + Debug + Send + Sync {}
