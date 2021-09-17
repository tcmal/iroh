//! Traits and structs related to the representation of kinds.
use std::{
    iter::{Iterator, once, Once, Chain},
    marker::PhantomData
};


/// A type which can be reified into an iterator over `KindDesc`s.
/// A kind of object present in a container. This is the type your code will eventually interact
/// with, and it's also used to get details about how the kind should be shown in the editor.
pub trait Kind {
    fn kind_desc() -> KindDesc;
}

/// A type-level list of multiple `Kind` types.
/// Any `Kind` implements this, so you'll often only need your own kinds and `ConsKinds`.
pub trait KindsCollection {
    type Iterator: Iterator<Item = KindDesc>;
    fn kinds() -> Self::Iterator;
}

/// Describes a kind of editable object
pub struct KindDesc {
    // TODO
}

impl<X: Kind> KindsCollection for X {
    type Iterator = Once<KindDesc>;
    fn kinds() -> Self::Iterator {
       once(Self::kind_desc()) 
    }
    
}

/// Used when you have multiple possible kinds in a single `KindsCollection`. This can be used
/// recursively to construct an arbitrary-length type level list of Kinds. Note that this should
/// never actually be constructed.
pub struct ConsKinds<A, B> (PhantomData<(A, B)>);
impl<A: KindsCollection, B: KindsCollection> KindsCollection for ConsKinds<A, B> {
    type Iterator = Chain<A::Iterator, B::Iterator>;
    fn kinds() -> Self::Iterator {
        A::kinds().chain(B::kinds())
    }
}


