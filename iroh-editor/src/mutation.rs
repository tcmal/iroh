use dyn_clone::{clone_trait_object, DynClone};
use std::fmt::Debug;

pub trait Mutator<T>: Debug + Send + DynClone {
    fn mutate(&self, target: &mut T);
}

clone_trait_object!(<T> Mutator<T>);

pub trait Lens: Debug + Send + Clone {
    type Source;
    type Target;
    fn get(&self, source: &Self::Source) -> &Self::Target;
    fn get_mut(&self, source: &mut Self::Source) -> &mut Self::Target;
}

#[derive(Debug, Clone)]
pub struct CompositeLens<A: Lens, B: Lens>(A, B);
impl<A: Lens, B: Lens<Source = A::Target>> Lens for CompositeLens<A, B> {
    type Source = A::Source;
    type Target = B::Target;

    fn get(&self, source: &Self::Source) -> &Self::Target {
        self.1.get(self.0.get(source))
    }

    fn get_mut(&self, source: &mut Self::Source) -> &mut Self::Target {
        self.1.get_mut(self.0.get_mut(source))
    }
}

#[derive(Debug, Clone)]
pub struct LensMutator<L: Lens>(L, L::Target);
impl<L: Lens> LensMutator<L> {
    pub fn new(lens: L, new: L::Target) -> Self {
        Self(lens, new)
    }
}
impl<S, T: Debug + Clone + Send, L: Lens<Source = S, Target = T>> Mutator<S> for LensMutator<L> {
    fn mutate(&self, target: &mut S) {
        *self.0.get_mut(target) = self.1.clone()
    }
}
