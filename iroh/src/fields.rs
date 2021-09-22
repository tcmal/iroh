//! Traits and structs related to the representation of fields.

/// A type which can be reified into an iterator over `KindDesc`s.
/// A kind of object present in a container. This is the type your code will eventually interact
/// with, and it's also used to get details about how the kind should be shown in the editor.
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
