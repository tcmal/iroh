use crate::{Key, Kind, ObjectStore};

/// Vector backed container
pub struct VecContainer<K>(Vec<K>);
impl<E: Key + Ord, K: Kind<Key = E>> ObjectStore<K> for VecContainer<K> {
    type AllIter<'a> = std::slice::Iter<'a, K>;
    fn all<'a>(&'a self) -> Self::AllIter<'a> {
        self.0.iter()
    }
    fn empty() -> Self {
        Self(vec![])
    }

    fn exists(&self, key: &K::Key) -> bool {
        self.0.iter().find(|x| x.key() == key).is_some()
    }

    fn count(&self) -> usize {
        self.0.len()
    }

    fn new(&mut self) -> &K::Key {
        let last = self.0.iter().map(|x| x.key()).max();
        let next = if let Some(last) = last {
            K::Key::next(last)
        } else {
            K::Key::first()
        };
        self.0.push(K::default_with_key(next));

        self.0.last().unwrap().key()
    }

    fn get(&self, key: &K::Key) -> Option<&K> {
        self.0.iter().find(|x| x.key() == key)
    }

    fn get_mut(&mut self, key: &K::Key) -> Option<&mut K> {
        self.0.iter_mut().find(|x| x.key() == key)
    }
}
