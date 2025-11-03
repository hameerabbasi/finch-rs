use std::{fmt::Debug, marker::PhantomData};

use num::{Integer, NumCast, ToPrimitive};

use super::ConcreteLevel;

#[derive(Debug, Clone)]
pub struct Element<I>
where
    I: Integer + Debug + Clone + NumCast + ToPrimitive,
{
    phantom: PhantomData<I>,
}

impl<I> ConcreteLevel<'static, I> for Element<I>
where
    I: Integer + Debug + Clone + NumCast + ToPrimitive,
{
    #[inline(always)]
    fn get_level_arrays(&self) -> Vec<&Vec<I>> {
        vec![]
    }

    #[inline(always)]
    fn get_shape(&self) -> Vec<I> {
        vec![]
    }

    #[inline(always)]
    fn get_inner(&self) -> Option<&'static dyn ConcreteLevel<'static, I>> {
        None
    }

    #[inline(always)]
    fn get_nnz(&self) -> usize {
        1
    }
}
