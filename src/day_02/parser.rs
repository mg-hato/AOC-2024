use crate::{helper::result, parser::Parse, reader::Line};

use super::models::{LevelReport, LevelReports};


pub struct LevelReportsParser {
    line_re: regex::Regex
}

mod error {
    use std::num::ParseIntError;

    const PREFIX : &str = "[Parser D-02]";

    pub fn regex_error(e: regex::Error) -> String {
        format!("{} could not compile line regex {}", PREFIX, e)
    }

    pub fn parse_line_error(line_num: usize) -> String {
        format!("{} line #{} does not match the regex", PREFIX, line_num)
    }

    pub fn parse_number_error(num: &str, line_num: usize, e: ParseIntError) -> String {
        format!("{} failed to parse number '{}' on line #{} because of parsing error '{}'", PREFIX, num, line_num, e)
    }
}

impl LevelReportsParser {
    pub fn new() -> Result<LevelReportsParser, String> {
        regex::Regex::new(r"^(?:\d+ *)+$")
            .map_err(error::regex_error)
            .map(|line_re|LevelReportsParser{line_re})
    }

    fn parse_num(num: &str, line_num: usize) -> Result<u32, String> {
        num.parse::<u32>()
            .map_err(|e|error::parse_number_error(num, line_num, e))
    }

    fn parse_line(&self, line: Line) -> Result<LevelReport, String> {
        if !self.line_re.is_match(&line.text()) {
            Err(error::parse_line_error(line.number()))
        } else {
            let parsed_numbers = line.text()
                .split(" ")
                .filter(|word| word.len() > 0)
                .map(|word|Self::parse_num(word, line.number()))
                .collect();

            result::collect(parsed_numbers)
                .map(LevelReport::new)
        }
    }
}

impl Parse<LevelReports> for LevelReportsParser {
    fn parse(&self, lines: crate::reader::VecLine) -> Result<LevelReports, String> {
        result::collect(lines.into_iter().map(|line|self.parse_line(line)).collect())
            .map(LevelReports::new)
    }
}