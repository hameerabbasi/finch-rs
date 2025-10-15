use std::fmt::Debug;
use std::rc::Rc;

use dyn_clone::DynClone;
use dyn_eq::DynEq;

pub trait ConcreteLevel: DynClone + DynEq + Debug {
    fn get_level_arrays(&self) -> Vec<Vec<u8>>;
    fn get_shape(&self) -> Vec<u64>;
    fn get_inner(&self) -> Rc<dyn ConcreteLevel>;

    fn get_all_arrays(&self) -> Vec<Vec<u8>> {
        let mut a = self.get_level_arrays();
        a.extend((*self.get_inner()).get_all_arrays());
        a
    }
}

dyn_clone::clone_trait_object!(ConcreteLevel);
dyn_eq::eq_trait_object!(ConcreteLevel);
