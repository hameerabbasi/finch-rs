use std::{cell::RefCell, collections::HashMap, hash::BuildHasherDefault, rc::Rc};

use seahash::SeaHasher;

pub type SeaHashMap = RefCell<HashMap<Term, Option<Term>, BuildHasherDefault<SeaHasher>>>;

use super::term::Term;

pub trait Rewriter {
    fn rewrite(&self, x: &Term) -> Option<Term>;
}

pub struct Default<'a, R: Rewriter> {
    rw: &'a R,
}

impl<'a, R: Rewriter> From<&'a R> for Default<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self { rw }
    }
}

impl<'a, R: Rewriter> Rewriter for Default<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        Some(self.rw.rewrite(x).unwrap_or(x.clone()))
    }
}

pub struct PreWalk<'a, R: Rewriter> {
    rw: &'a R,
}

impl<'a, R: Rewriter> From<&'a R> for PreWalk<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self { rw }
    }
}

impl<'a, R: Rewriter> Rewriter for PreWalk<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        let y = self.rw.rewrite(x);
        y.map(|y| match y {
            Term::Leaf(_) => y,
            Term::Tree(y) => {
                let args = y.children();
                y.make(
                    args.iter()
                        .map(|yi| Default::from(self).rewrite(yi).unwrap())
                        .collect(),
                )
            }
        })
        .or_else(|| match x {
            Term::Leaf(_) => None,
            Term::Tree(x) => {
                let args = x.children();
                let new_args: Vec<_> = args.iter().map(|ai| self.rewrite(ai)).collect();
                if !new_args.iter().all(Option::is_none) {
                    None
                } else {
                    let args = new_args
                        .iter()
                        .zip(args)
                        .map(|(a1i, a2i)| a1i.clone().unwrap_or(a2i))
                        .collect();
                    Some(x.make(args))
                }
            }
        })
    }
}

pub struct PostWalk<'a, R: Rewriter> {
    rw: &'a R,
}

impl<'a, R: Rewriter> From<&'a R> for PostWalk<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self { rw }
    }
}

impl<'a, R: Rewriter> Rewriter for PostWalk<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        match x {
            Term::Leaf(_) => self.rw.rewrite(x),
            Term::Tree(xt) => {
                let args = xt.children();
                let new_args: Vec<_> = args.iter().map(|ai| self.rewrite(ai)).collect();
                if new_args.iter().all(Option::is_none) {
                    self.rw.rewrite(x)
                } else {
                    let args = new_args
                        .iter()
                        .zip(args)
                        .map(|(a1i, a2i)| a1i.clone().unwrap_or(a2i))
                        .collect();
                    Some(xt.make(args))
                }
            }
        }
    }
}

pub struct Chain {
    rws: Vec<Rc<dyn Rewriter>>,
}

impl<T: IntoIterator<Item = Rc<dyn Rewriter>>> From<T> for Chain {
    fn from(rws: T) -> Self {
        Chain {
            rws: rws.into_iter().collect(),
        }
    }
}

impl Rewriter for Chain {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        let mut x = x.clone();
        let mut is_success = false;
        self.rws.iter().for_each(|rw| {
            let y = rw.rewrite(&x);
            if let Some(y) = y {
                x = y;
                is_success = true;
            }
        });
        if is_success { Some(x) } else { None }
    }
}

pub struct FixPoint<'a, R: Rewriter> {
    rw: &'a R,
}

impl<'a, R: Rewriter> From<&'a R> for FixPoint<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self { rw }
    }
}

impl<'a, R: Rewriter> Rewriter for FixPoint<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        let mut x = x.clone();
        self.rw.rewrite(&x).map(|y| {
            let mut y = Some(y);
            while y.as_ref().is_some_and(|y| x != *y) {
                x = y.unwrap();
                y = self.rw.rewrite(&x);
            }
            x
        })
    }
}

pub struct PreStep<'a, R: Rewriter> {
    rw: &'a R,
}

impl<'a, R: Rewriter> From<&'a R> for PreStep<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self { rw }
    }
}

impl<'a, R: Rewriter> Rewriter for PreStep<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        let y = self.rw.rewrite(x);
        y.map(|y| match y {
            Term::Leaf(_) => y,
            Term::Tree(y) => {
                let args = y
                    .children()
                    .into_iter()
                    .map(|ai| self.rewrite(&ai).unwrap_or(ai))
                    .collect();
                y.make(args)
            }
        })
    }
}

pub struct Memo<'a, R: Rewriter> {
    rw: &'a R,
    cache: SeaHashMap,
}

impl<'a, R: Rewriter> From<&'a R> for Memo<'a, R> {
    fn from(rw: &'a R) -> Self {
        Self {
            rw,
            cache: RefCell::new(HashMap::default()),
        }
    }
}

impl<'a, R: Rewriter> Rewriter for Memo<'a, R> {
    fn rewrite(&self, x: &Term) -> Option<Term> {
        self.cache
            .borrow_mut()
            .entry(x.clone())
            .or_insert_with(|| self.rw.rewrite(x))
            .clone()
    }
}
