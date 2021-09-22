use crate::{Kind, ObjectContainer};

/// Vector backed container
pub struct VecContainer<K>(Vec<K>);
impl<'a, K: 'a + Kind> ObjectContainer<'a, K> for VecContainer<K> {
    type AllIter = std::slice::Iter<'a, K>;
    fn all(&'a self) -> Self::AllIter {
        self.0.iter()
    }
    fn empty() -> Self {
        Self(vec![])
    }
}
