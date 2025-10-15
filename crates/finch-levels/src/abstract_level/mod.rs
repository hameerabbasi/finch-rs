use std::{fmt::Debug, rc::Rc};

use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;

mod dense;
mod element;
mod sparse_vector;

pub use dense::Dense;
pub use element::Element;
pub use sparse_vector::SparseVector;

pub trait AbstractLevel: DynClone + DynEq + DynHash + Debug {
    fn get_ndim(&self) -> u64;
    fn get_inner(&self) -> Option<Rc<dyn AbstractLevel>>;
}

dyn_clone::clone_trait_object!(AbstractLevel);
dyn_hash::hash_trait_object!(AbstractLevel);
dyn_eq::eq_trait_object!(AbstractLevel);
