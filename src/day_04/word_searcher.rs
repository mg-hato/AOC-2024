
use crate::{answer::{Answer, DisplayableAnswer}, helper::table::Table, solver::Solve};

use super::find::Find;


pub struct WordSearcher<F> where F: Find + 'static {
    finder_fn: Box<dyn Fn(Table<char>) -> F>
}

impl <F> WordSearcher<F> where F: Find + 'static {
    pub fn new<FF>(finder_fn: FF) -> WordSearcher<F>
    where FF: Fn(Table<char>) -> F + 'static {
        WordSearcher { finder_fn: Box::new(finder_fn) }
    }
}

impl <F> Solve<Table<char>> for WordSearcher<F> where F: Find + 'static {
    fn solve(&self, input: Table<char>) -> Result<Answer, String> {
        let finder = (self.finder_fn)(input);
        Ok(DisplayableAnswer::new(finder.find_all()))
    }
}