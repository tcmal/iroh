use dyn_clone::{clone_trait_object, DynClone};
use std::{fmt::Debug, marker::PhantomData};

pub trait Mutator<T>: Debug + Send + DynClone {
    fn mutate(&self, target: &mut T);
}

clone_trait_object!(<T> Mutator<T>);

pub trait Lens: Debug + Send + Clone {
    type Source;
    type Target;
    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target;
    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target;
}

#[derive(Debug, Clone)]
pub struct CompositeLens<A: Lens, B: Lens>(PhantomData<(A, B)>);
impl<A: Lens<Target = AT>, AT: 'static, B: Lens<Source = A::Target>> Lens for CompositeLens<A, B> {
    type Source = A::Source;
    type Target = B::Target;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        B::get(A::get(source))
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        B::get_mut(A::get_mut(source))
    }
}

#[derive(Debug, Clone)]
pub struct LensMutator<L: Lens>(L::Target);
impl<L: Lens> LensMutator<L> {
    pub fn new(new: L::Target) -> Self {
        Self(new)
    }
}
impl<S, T: Debug + Clone + Send, L: Lens<Source = S, Target = T>> Mutator<S> for LensMutator<L> {
    fn mutate(&self, target: &mut S) {
        *L::get_mut(target) = self.0.clone()
    }
}
