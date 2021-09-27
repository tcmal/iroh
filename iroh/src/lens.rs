use std::{fmt::Debug, marker::PhantomData};

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
/// A lens that simply returns the object given to it.
#[derive(Debug, Clone)]
pub struct RootLens<A>(PhantomData<A>);
impl<A: Debug + Clone + Send> Lens for RootLens<A> {
    type Source = A;
    type Target = A;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        source
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        source
    }
}

/// Get the first element of a two-object tuple.
#[derive(Debug, Clone)]
pub struct TupleHeadLens<A, B>(PhantomData<(A, B)>);
impl<A: Send + Debug + Clone, B: Send + Debug + Clone> Lens for TupleHeadLens<A, B> {
    type Source = (A, B);
    type Target = A;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.0
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.0
    }
}

/// Get the second element of a two-object tuple.
#[derive(Debug, Clone)]
pub struct TupleTailLens<A, B>(PhantomData<(A, B)>);
impl<A: Send + Debug + Clone, B: Send + Debug + Clone> Lens for TupleTailLens<A, B> {
    type Source = (A, B);
    type Target = B;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.1
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.1
    }
}
