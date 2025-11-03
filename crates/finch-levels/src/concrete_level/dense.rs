use std::fmt::Debug;

use num::{Integer, NumCast, ToPrimitive};

use super::ConcreteLevel;

#[derive(Debug, Clone)]
pub struct Dense<'a, I>
where
    I: Integer + Debug + Clone + NumCast + ToPrimitive,
{
    dim: I,
    inner: &'a dyn ConcreteLevel<'a, I>,
}

impl<'a, I> ConcreteLevel<'a, I> for Dense<'a, I>
where
    I: Integer + Debug + Clone + NumCast + ToPrimitive,
{
    #[inline(always)]
    fn get_level_arrays(&self) -> Vec<&Vec<I>> {
        vec![]
    }

    #[inline(always)]
    fn get_shape(&self) -> Vec<I> {
        let mut v = vec![self.dim.clone()];
        v.extend(self.inner.get_shape());
        v
    }

    #[inline(always)]
    fn get_inner(&self) -> Option<&'a dyn ConcreteLevel<'a, I>> {
        Some(self.inner)
    }

    #[inline(always)]
    fn get_nnz(&self) -> usize {
        self.dim
            .to_usize()
            .unwrap()
            .checked_mul(self.inner.get_nnz())
            .unwrap()
    }
}
