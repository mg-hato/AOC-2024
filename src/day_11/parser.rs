use crate::{helper::result::collect, parser::Parse, reader::{Line, VecLine}};

use super::model::Stones;

pub struct StonesParser {
    stones_re: regex::Regex,
}

mod error {
    use std::num::ParseIntError;

    use crate::reader::Line;

    const PREFIX: &str = "[Parser D-11]";

    pub fn stones_re_error(e: regex::Error) -> String {
        format!("{} could not create parser due to regex error {}", PREFIX, e)
    }

    pub fn not_single_line_error(lines_count: usize) -> String {
        format!("{} expected exactly one line but {} were provided", PREFIX, lines_count)
    }

    pub fn line_pattern_error(line: &Line) -> String {
        format!("{} input line '{}' does not satisfy regex pattern", PREFIX, line.text)
    }

    pub fn number_parse_error(num: &str, e: ParseIntError) -> String {
        format!("{} could not parse number '{}' due to parsing error '{}'", PREFIX, num, e)
    }
}

impl StonesParser {
    pub fn new() -> Result<StonesParser, String> {
        regex::Regex::new(r"\d+(?: +\d+)*")
            .map(|stones_re|StonesParser { stones_re })
            .map_err(error::stones_re_error)
    }

    fn parse_line(&self, line: &Line) -> Result<Stones, String> {
        if !self.stones_re.is_match(&line.text) {
            return Err(error::line_pattern_error(line));
        }

        let parsed_numbers = line.text.split(' ')
            .filter(|s|!s.is_empty())
            .map(|num|num.parse().map_err(|e|error::number_parse_error(num, e)))
            .collect();

        collect(parsed_numbers).map(Stones)
    }
}

impl Parse<Stones> for StonesParser {
    fn parse(&self, vec_line: VecLine) -> Result<Stones, String> {
        if vec_line.lines.len() != 1 {
            Err(error::not_single_line_error(vec_line.lines.len()))
        } else {
            self.parse_line(&vec_line.lines[0])
        }
    }
}