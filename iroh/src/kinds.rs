//! Traits and structs related to the representation of kinds.
use std::fmt::Debug;

use iced::Element;

use crate::Message;

/// A type of object contained by a [`ObjectStore`]
pub trait Kind: 'static + Clone + Debug {
    type Key: Key;
    type Field: Field<Kind = Self>;

    fn key(&self) -> Self::Key;
    fn default_with_key(key: Self::Key) -> Self;
}

/// A single editable part of a Kind
pub trait Field: Default {
    type Kind: Kind;

    fn view(&mut self, val: &Self::Kind) -> Vec<Element<Message<Self::Kind>>>;
}

/// One field, then the other.
pub struct ConsFields<A, B>(A, B);
impl<K: Kind, A: Field<Kind = K>, B: Field<Kind = K>> Default for ConsFields<A, B> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<K: Kind, A: Field<Kind = K>, B: Field<Kind = K>> Field for ConsFields<A, B> {
    type Kind = K;

    fn view(&mut self, val: &Self::Kind) -> Vec<Element<Message<Self::Kind>>> {
        let mut v = self.0.view(val);
        v.extend(self.1.view(val));

        v
    }
}

/// Uniquely identifies a Kind in a store.
pub trait Key: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
impl<T> Key for T where T: 'static + Copy + Clone + Debug + Send + Sync + PartialEq {}
