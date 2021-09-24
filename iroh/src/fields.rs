//! Traits and structs related to the representation of fields.
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
