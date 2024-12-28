use crate::{parser::Parse, reader::VecLine};


mod error {
    const PREFIX: &str = "[Parser D-04]";

    pub fn regex_error(e: regex::Error) -> String {
        format!("{} error when creating line regex '{}'", PREFIX, e)
    }

    pub fn match_line_error(line_num: usize) -> String {
        format!("{} line #{} does not match expected pattern", PREFIX, line_num)
    }
}

pub struct WordSearchParser {
    line_re: regex::Regex
}

impl WordSearchParser {
    pub fn new() -> Result<WordSearchParser, String> {
        regex::Regex::new(r"^[XMAS]*$")
            .map(|line_re|WordSearchParser{ line_re })
            .map_err(error::regex_error)
    }
}

impl Parse<VecLine> for WordSearchParser {
    fn parse(&self, vec_line: VecLine) -> Result<VecLine, String> {
        for line in vec_line.lines.iter() {
            if !self.line_re.is_match(&line.text) {
                return Err(error::match_line_error(line.number))
            }
        }
        Ok(vec_line)
    }
}