use std::fmt::{Debug, Display};
use crate::reader::VecLine;

pub type Parser<T> = Box<dyn Parse<T>>;

pub trait Parse<T: Eq + Display + Clone + Debug>
{
    fn parse(&self, vec_line: VecLine) -> Result<T, String>;
}

pub struct TrivialParser;

impl TrivialParser {
    pub fn new() -> TrivialParser { TrivialParser }
}

impl Parse<VecLine> for TrivialParser {
    fn parse(&self, lines: VecLine) -> Result<VecLine, String> {
        Ok(lines)
    }
}