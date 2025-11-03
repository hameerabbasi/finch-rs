use std::{fmt::Debug, rc::Rc};

use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;

pub mod dense;
pub mod element;
pub mod sparse_vector;

pub trait AbstractLevel: DynClone + DynEq + DynHash + Debug {
    fn get_ndim(&self) -> u64;
    fn get_inner(&self) -> Option<Rc<dyn AbstractLevel>>;

    #[inline(always)]
    fn is_leaf(&self) -> bool {
        self.get_inner().is_none()
    }
}

dyn_clone::clone_trait_object!(AbstractLevel);
dyn_hash::hash_trait_object!(AbstractLevel);
dyn_eq::eq_trait_object!(AbstractLevel);
