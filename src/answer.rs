use std::fmt::{Debug, Display};

/// An answer/solution abstraction.
pub type Answer = Box<dyn Report>;

pub trait Report : Debug {
    fn report(&self) -> String;
}

/// A common shorthand for a container that implements the `Answer` abstraction
#[derive(Eq, PartialEq, Debug)]
pub struct DisplayableAnswer<T: Display + Eq + PartialEq + 'static> {
    answer: T
}

impl <T: Display + Eq + PartialEq + Debug + 'static> DisplayableAnswer<T> {
    pub fn new(answer: T) -> Answer {
        Box::new(DisplayableAnswer{ answer })
    }
}

impl <T: Display + Eq + PartialEq + Debug + 'static> Report for DisplayableAnswer<T> {
    fn report(&self) -> String {
        format!("The answer is: {}\n", self.answer)
    }
}