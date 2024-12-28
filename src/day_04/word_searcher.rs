
use crate::{answer::DisplayableAnswer, reader::VecLine, solver::Solve};

use super::find::Find;


pub struct WordSearcher<F> where F: Find + 'static {
    finder_fn: Box<dyn Fn(&VecLine) -> F>
}

impl <F> WordSearcher<F> where F: Find + 'static {
    pub fn new<FF>(finder_fn: FF) -> WordSearcher<F>
    where FF: Fn(&VecLine) -> F + 'static {
        WordSearcher { finder_fn: Box::new(finder_fn) }
    }
}

impl <F> Solve<VecLine> for WordSearcher<F> where F: Find + 'static {
    fn solve(&self, input: VecLine) -> Result<crate::answer::Answer, String> {
        let finder = (self.finder_fn)(&input);
        Ok(DisplayableAnswer::new(finder.find_all()))
    }
}