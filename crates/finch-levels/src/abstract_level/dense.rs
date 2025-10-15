use std::rc::Rc;

use super::AbstractLevel;

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Hash, Eq)]
pub struct Dense {
    depth: u64,
    inner: Rc<dyn AbstractLevel>,
}

impl AbstractLevel for Dense {
    #[inline(always)]
    fn get_ndim(&self) -> u64 {
        self.depth + self.inner.get_ndim()
    }

    #[inline(always)]
    fn get_inner(&self) -> Option<Rc<dyn AbstractLevel>> {
        Some(self.inner.clone())
    }
}

impl PartialEq for Dense {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth && *self.inner == *other.inner
    }
}
