use std::fmt::Display;

use crate::helper;

pub type Reader = Box<dyn Read>;

pub trait Read {
    fn read(&self, input_file_path: &str) -> Result<VecLine, String>;
}

/// A line is some text read from a file, and the row number of the line inside the file
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Line {
    pub text: String,
    pub number: usize,
}

impl Line {
    pub fn new(text: String, number: usize) -> Line {
        Line { text, number }
    }

    pub fn textf(&self) -> String {
        self.text.to_owned()
    }

    pub fn numberf(&self) -> usize {
        self.number
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[#{},'{}']", self.number, self.text)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VecLine {
    pub lines: Vec<Line>
}

impl VecLine {
    pub fn new(lines: Vec<Line>) -> VecLine  { VecLine{ lines } }
}

impl Display for VecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", helper::display::vector_display(&self.lines, ","))
    }
}