//! Code related to mutation of generic [`crate::Kind`]s.

use dyn_clone::{clone_trait_object, DynClone};
use std::{fmt::Debug, marker::PhantomData};

/// Describes a mutation that should be applied to an object
pub trait Mutator<T>: Debug + Send + DynClone {
    /// Apply the mutation to the given target.
    fn apply(self, target: &mut T);
}

clone_trait_object!(<T> Mutator<T>);

/// A way to get a (mutable) reference into a generic object.
/// Note that most lenses need never be constructed.
pub trait Lens: Debug + Send + Clone {
    /// The 'root' object this accesses
    type Source;

    /// The type it borrows
    type Target;

    /// Get an immutable borrow of the targeted attribute
    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target;

    /// Get a mutable borrow of the targeted attribute
    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target;
}

/// Applies lens A, then lens B.
#[derive(Debug, Clone)]
pub struct CompositeLens<A: Lens, B: Lens>(PhantomData<(A, B)>);
impl<A: Lens<Target = AT>, AT: 'static, B: Lens<Source = A::Target>> Lens for CompositeLens<A, B> {
    type Source = A::Source;
    type Target = B::Target;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        B::get(A::get(source))
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        B::get_mut(A::get_mut(source))
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
    fn apply(self, target: &mut S) {
        *L::get_mut(target) = self.0.clone()
    }
}
