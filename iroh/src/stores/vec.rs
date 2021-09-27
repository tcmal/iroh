use crate::{Field, Key, Kind, ObjectStore};

/// Vector backed container
pub struct VecContainer<K: Kind>(Vec<(K::Key, K, <K::Field as Field>::WorkingValues)>);
impl<E: Key + Ord, K: Kind<Key = E>> ObjectStore<K> for VecContainer<K> {
    type Items<'a> =
        PopTupleRefs<std::slice::Iter<'a, (K::Key, K, <K::Field as Field>::WorkingValues)>>;
    fn items<'a>(&'a self) -> Self::Items<'a> {
        PopTupleRefs::new(self.0.iter())
    }
    fn empty() -> Self {
        Self(vec![])
    }

    fn exists(&self, key: &K::Key) -> bool {
        self.0.iter().find(|(k, _, _)| k == key).is_some()
    }

    fn count(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self) -> &K::Key {
        let last = self.0.iter().map(|(k, _, _)| k).max();
        let next = if let Some(last) = last {
            K::Key::next(last)
        } else {
            K::Key::first()
        };
        self.0.push((
            next,
            K::default(),
            <K::Field as Field>::WorkingValues::default(),
        ));

        &self.0.last().unwrap().0
    }

    fn get(&self, key: &K::Key) -> Option<(&K, &<K::Field as Field>::WorkingValues)> {
        self.0
            .iter()
            .find(|(k, _, _)| k == key)
            .map(|(_, v, w)| (v, w))
    }

    fn get_mut(
        &mut self,
        key: &K::Key,
    ) -> Option<(&mut K, &mut <K::Field as Field>::WorkingValues)> {
        self.0
            .iter_mut()
            .find(|(k, _, _)| k == key)
            .map(|(_, v, w)| (v, w))
    }

    type Keys<'a> =
        FirstTupleElem<std::slice::Iter<'a, (K::Key, K, <K::Field as Field>::WorkingValues)>>;

    fn keys<'a>(&'a self) -> Self::Keys<'a> {
        FirstTupleElem::new(self.0.iter())
    }

    type Values<'a> =
        SecondTupleElem<std::slice::Iter<'a, (K::Key, K, <K::Field as Field>::WorkingValues)>>;

    fn values<'a>(&'a self) -> Self::Values<'a> {
        SecondTupleElem::new(self.0.iter())
    }
}

pub struct PopTupleRefs<T>(T);
impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> PopTupleRefs<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> Iterator for PopTupleRefs<T> {
    type Item = (&'a A, &'a B, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, b, c)| (a, b, c))
    }
}

pub struct FirstTupleElem<T>(T);
impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> FirstTupleElem<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> Iterator for FirstTupleElem<T> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, _b, _c)| a)
    }
}

pub struct SecondTupleElem<T>(T);
impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> SecondTupleElem<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<'a, A: 'a, B: 'a, C: 'a, T: Iterator<Item = &'a (A, B, C)>> Iterator for SecondTupleElem<T> {
    type Item = &'a B;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_a, b, _c)| b)
    }
}
