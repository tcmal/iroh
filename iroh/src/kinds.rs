//! Traits and structs related to the representation of kinds.
use std::fmt::Debug;

/// A type of object contained by a [`ObjectStore`]
pub trait Kind: 'static + Clone + Debug {
    type Key: Key;

    fn key(&self) -> Self::Key;
    fn default_with_key(key: Self::Key) -> Self;
}

/// A single editable part of a Kind
pub trait Field {
    fn new() -> Self;
}

/// One field, then the other.
pub struct ConsFields<A, B>(A, B);
impl<A: Field, B: Field> Field for ConsFields<A, B> {
    fn new() -> Self {
        Self(A::new(), B::new())
    }
}

/// Uniquely identifies a Kind in a store.
pub trait Key: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
impl<T> Key for T where T: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
