use std::rc::Rc;

use super::AbstractLevel;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Element {}

impl AbstractLevel for Element {
    #[inline(always)]
    fn get_ndim(&self) -> u64 {
        0
    }

    #[inline(always)]
    fn get_inner(&self) -> Option<Rc<dyn AbstractLevel>> {
        None
    }
}
