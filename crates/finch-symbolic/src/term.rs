use std::rc::Rc;

use dyn_eq::DynEq;
use dyn_hash::DynHash;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Leaf(Rc<dyn Leaf>),
    Tree(Rc<dyn Tree>),
}

pub trait Leaf: DynEq + DynHash {}
dyn_eq::eq_trait_object!(Leaf);
dyn_hash::hash_trait_object!(Leaf);

pub trait Tree: DynEq + DynHash {
    fn children(&self) -> Vec<Term>;
    fn make(&self, children: Vec<Term>) -> Term;
}
dyn_eq::eq_trait_object!(Tree);
dyn_hash::hash_trait_object!(Tree);

impl Term {
    pub fn pre_order_dfs(&self) -> Vec<Term>
    where
        Self: Clone,
    {
        match self {
            Term::Leaf(_) => vec![self.clone()],
            Term::Tree(t) => {
                let mut v = vec![self.clone()];
                for l in t.children() {
                    v.extend_from_slice(l.pre_order_dfs().as_slice());
                }
                v
            }
        }
    }

    pub fn post_order_dfs(&self) -> Vec<Term>
    where
        Self: Clone,
    {
        match self {
            Term::Leaf(_) => vec![self.clone()],
            Term::Tree(t) => {
                let mut v = vec![];
                for l in t.children() {
                    v.extend_from_slice(l.post_order_dfs().as_slice());
                }
                v.push(self.clone());
                v
            }
        }
    }
}
