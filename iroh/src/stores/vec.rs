use crate::{Key, Kind, ObjectStore};

/// Vector backed container
pub struct VecContainer<K: Kind>(Vec<(K::Key, K)>);
impl<E: Key + Ord, K: Kind<Key = E>> ObjectStore<K> for VecContainer<K> {
    type Items<'a> = PopTupleRefs<std::slice::Iter<'a, (K::Key, K)>>;
    fn items<'a>(&'a self) -> Self::Items<'a> {
        PopTupleRefs::new(self.0.iter())
    }
    fn empty() -> Self {
        Self(vec![])
    }

    fn exists(&self, key: &K::Key) -> bool {
        self.0.iter().find(|(k, _)| k == key).is_some()
    }

    fn count(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self) -> &K::Key {
        let last = self.0.iter().map(|(k, _)| k).max();
        let next = if let Some(last) = last {
            K::Key::next(last)
        } else {
            K::Key::first()
        };
        self.0.push((next, K::default()));

        &self.0.last().unwrap().0
    }

    fn get(&self, key: &K::Key) -> Option<&K> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    fn get_mut(&mut self, key: &K::Key) -> Option<&mut K> {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    type Keys<'a> = FirstTupleElem<std::slice::Iter<'a, (K::Key, K)>>;

    fn keys<'a>(&'a self) -> Self::Keys<'a> {
        FirstTupleElem::new(self.0.iter())
    }

    type Values<'a> = SecondTupleElem<std::slice::Iter<'a, (K::Key, K)>>;

    fn values<'a>(&'a self) -> Self::Values<'a> {
        SecondTupleElem::new(self.0.iter())
    }
}

pub struct PopTupleRefs<T>(T);
impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> PopTupleRefs<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> Iterator for PopTupleRefs<T> {
    type Item = (&'a A, &'a B);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, b)| (a, b))
    }
}

pub struct FirstTupleElem<T>(T);
impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> FirstTupleElem<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> Iterator for FirstTupleElem<T> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, _b)| a)
    }
}

pub struct SecondTupleElem<T>(T);
impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> SecondTupleElem<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, T: Iterator<Item = &'a (A, B)>> Iterator for SecondTupleElem<T> {
    type Item = &'a B;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_a, b)| b)
    }
}
