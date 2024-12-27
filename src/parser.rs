use std::fmt::{Debug, Display};
use crate::reader::VecLine;

pub type Parser<T> = Box<dyn Parse<T>>;

pub trait Parse<T: Eq + Display + Clone + Debug>
{
    fn parse(&self, lines: VecLine) -> Result<T, String>;
}