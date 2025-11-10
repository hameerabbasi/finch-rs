use std::fmt::Debug;

use dyn_clone::DynClone;
use num::Integer;
use num::NumCast;
use num::ToPrimitive;

pub mod dense;
pub mod element;
pub mod sparse_vector;

pub trait ConcreteLevel<'a, I>: DynClone + Debug
where
    I: Integer + Debug + Clone + NumCast + ToPrimitive,
{
    fn get_level_arrays(&'a self) -> Vec<&'a Vec<I>>;
    fn get_shape(&self) -> Vec<I>;
    fn get_inner(&self) -> Option<&'a dyn ConcreteLevel<'a, I>>;
    fn get_nnz(&self) -> usize;

    fn get_all_arrays(&'a self) -> Vec<&'a Vec<I>> {
        let mut a = self.get_level_arrays();
        if let Some(inner) = self.get_inner() {
            a.extend(inner.get_all_arrays());
        }
        a
    }

    fn get_nelem(&self) -> I {
        self.get_shape()
            .into_iter()
            .fold(I::one(), |acc, x| acc * x)
    }
}

dyn_clone::clone_trait_object!(<'a, I> ConcreteLevel<'a, I> where I: Integer);
