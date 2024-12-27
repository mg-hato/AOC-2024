pub type Reader = Box<dyn Read>;

pub trait Read {
    fn read(&self, input_file_path: &str) -> Result<VecLine, String>;
}

/// A line is some text read from a file, and the row number of the line inside the file
#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    text: String,
    number: usize,
}

pub type VecLine = Vec<Line>;

impl Line {
    pub fn new(text: String, number: usize) -> Line {
        Line { text, number }
    }

    pub fn text(&self) -> String {
        self.text.to_owned()
    }

    pub fn number(&self) -> usize {
        self.number
    }
}
