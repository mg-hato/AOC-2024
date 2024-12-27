use crate::answer::Answer;

pub type Solver<T> = Box<dyn Solve<T>>;

pub trait Solve<T: Clone> {
    fn solve(&self, input: T) -> Result<Answer, String>;
}