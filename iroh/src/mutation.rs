//! Code related to mutation of generic [`crate::Kind`]s.

use crate::lens::Lens;
use dyn_clone::{clone_trait_object, DynClone};
use std::{fmt::Debug, marker::PhantomData};

/// Describes a mutation that should be applied to an object
pub trait Mutator<T>: Debug + Send + DynClone {
    /// Apply the mutation to the given target.
    fn apply(self: Box<Self>, target: &mut T);
}

clone_trait_object!(<T> Mutator<T>);

/// A mutator that does nothing.
#[derive(Debug, Clone)]
pub struct NopMutator;
impl<T> Mutator<T> for NopMutator {
    fn apply(self: Box<Self>, _target: &mut T) {
        ()
    }
}

/// A mutator which uses a lens to set the new value
#[derive(Debug, Clone)]
pub struct LensSet<L: Lens>(L::Target);
impl<L: Lens> LensSet<L> {
    pub fn new(new: L::Target) -> Self {
        Self(new)
    }
}
impl<S, T: Debug + Clone + Send, L: Lens<Source = S, Target = T>> Mutator<S> for LensSet<L> {
    fn apply(self: Box<Self>, target: &mut S) {
        *L::get_mut(target) = self.0.clone()
    }
}

/// Mutates an object by first applying a lens, then another mutator.
#[derive(Debug, Clone)]
pub struct InnerMutation<L: Lens>(Box<dyn Mutator<L::Target>>, PhantomData<L>);
impl<L: Lens> InnerMutation<L> {
    pub fn new(m: Box<dyn Mutator<L::Target>>) -> Self {
        Self(m, PhantomData)
    }
}
impl<L: Lens> Mutator<L::Source> for InnerMutation<L>
where
    L::Target: Debug + Clone + Send,
{
    fn apply(self: Box<Self>, target: &mut L::Source) {
        self.0.apply(L::get_mut(target))
    }
}
