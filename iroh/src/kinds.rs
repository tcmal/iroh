//! Traits and structs related to the representation of kinds.

use crate::{
    app::AppState,
    lens::{TupleHeadLens, TupleTailLens},
    mutation::InnerMutation,
    Message, ObjectStore,
};
use iced::Element;
use std::fmt::Debug;

/// A type of object contained by a [`ObjectStore`]
pub trait Kind: 'static + Clone + Debug + Default {
    type Key: Key;
    type Field: Field<Kind = Self>;
}

/// A single editable part of a Kind
pub trait Field: Default + Clone + Debug {
    type Kind: Kind;
    type WorkingValues: 'static + Default + Debug + Clone + Send;

    fn view<C: ObjectStore<Self::Kind>>(
        &mut self,
        key: &<<Self as Field>::Kind as Kind>::Key,
        val: &Self::Kind,
        app_state: &AppState<Self::Kind, C>,
        working: &<Self as Field>::WorkingValues,
    ) -> Vec<Element<Message<Self::Kind, Self::WorkingValues>>>;
}

/// One field, then the other.
#[derive(Debug, Clone)]
pub struct ConsFields<A, B>(A, B);
impl<K: Kind, A: Field<Kind = K>, B: Field<Kind = K>> Default for ConsFields<A, B> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<K: Kind, A: Field<Kind = K>, B: Field<Kind = K>> Field for ConsFields<A, B> {
    type Kind = K;
    type WorkingValues = (A::WorkingValues, B::WorkingValues);

    fn view<C: ObjectStore<Self::Kind>>(
        &mut self,
        key: &<<Self as Field>::Kind as Kind>::Key,
        val: &Self::Kind,
        app_state: &AppState<Self::Kind, C>,
        working: &<Self as Field>::WorkingValues,
    ) -> Vec<Element<Message<Self::Kind, Self::WorkingValues>>> {
        // Because a returns a message with working values A::WorkingValues, and b returns one with B::WorkingValues,
        // we need to map each one to our WorkingValues.
        // This is why we have a lot of big match arms. Technically we could `transmute`, but that's nasty.
        let a = self
            .0
            .view(key, val, app_state, &working.0)
            .into_iter()
            .map(|x| {
                x.map(|m| match m {
                    Message::Mutate(v, w) => Message::Mutate(
                        v,
                        Box::new(InnerMutation::<
                            TupleHeadLens<A::WorkingValues, B::WorkingValues>,
                        >::new(w)),
                    ),
                    Message::PaneMessage(m) => Message::PaneMessage(m),
                    Message::Select(s) => Message::Select(s),
                    Message::NewObject => Message::NewObject,
                    Message::Nop => Message::Nop,
                })
            });
        let b = self
            .1
            .view(key, val, app_state, &working.1)
            .into_iter()
            .map(|x| {
                x.map(|m| match m {
                    Message::Mutate(v, w) => Message::Mutate(
                        v,
                        Box::new(InnerMutation::<
                            TupleTailLens<A::WorkingValues, B::WorkingValues>,
                        >::new(w)),
                    ),
                    Message::PaneMessage(m) => Message::PaneMessage(m),
                    Message::Select(s) => Message::Select(s),
                    Message::NewObject => Message::NewObject,
                    Message::Nop => Message::Nop,
                })
            });

        a.chain(b).collect()
    }
}

/// Uniquely identifies a Kind in a store, and provides a way of generating keys for new objects.
/// These keys can be randomly generated, but see the documentation for `.next()` for the invariants
/// you need to satisfy.
pub trait Key: 'static + Clone + Debug + Send + Sync + PartialEq {
    /// Get the first key to use.
    fn first() -> Self;

    /// Get the next key, given the last one that was allocated.
    /// Assuming that all keys allocated beforehand are distinct, the key this function returns
    /// should also be distinct.
    /// For numbers, this can be as simple as incrementing.
    fn next(last: &Self) -> Self;
}
