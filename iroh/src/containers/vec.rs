use crate::{Kind, ObjectContainer};

/// Vector backed container
pub struct VecContainer<K>(Vec<K>);
impl<K: Kind> ObjectContainer<K> for VecContainer<K> {
    type AllIter<'a> = std::slice::Iter<'a, K>;
    fn all<'a>(&'a self) -> Self::AllIter<'a> {
        self.0.iter()
    }
    fn empty() -> Self {
        Self(vec![])
    }

    fn exists(&self, key: K::Key) -> bool {
        self.0.iter().find(|x| x.key() == key).is_some()
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}
