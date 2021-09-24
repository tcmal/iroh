use crate::{Kind, ObjectContainer};

/// Vector backed container
// TODO: Support more than just usize
pub struct VecContainer<K>(Vec<K>);
impl<K: Kind<Key = usize>> ObjectContainer<K> for VecContainer<K> {
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

    fn new(&mut self) -> K::Key {
        let key = self.0.iter().map(|x| x.key()).max().unwrap_or(0) + 1;
        self.0.push(K::default_with_key(key));

        key
    }

    fn get(&self, key: K::Key) -> Option<&K> {
        self.0.iter().find(|x| x.key() == key)
    }
}
