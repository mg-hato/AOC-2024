use crate::{helper::result::{self, collect}, parser::Parse, reader::{Line, VecLine}};

use super::model::{BytePosition, FallingBytes};


pub struct FallingBytesParser { byte_position_re: regex::Regex }

mod error {
    use std::num::ParseIntError;

    const PREFIX: &str = "[Parser D-18]";

    pub fn regex_create(e: regex::Error) -> String {
        format!("{} failed to create regex for byte position due to {}", PREFIX, e)
    }

    pub fn line_parse(line_num: usize) -> String {
        format!("{} line #{} does not match regex", PREFIX, line_num)
    }

    pub fn number_parse(line_num: usize, name: &str, e: ParseIntError) -> String {
        format!("{} line #{} could not parse number {} due to error: {}", PREFIX, line_num, name, e)
    }
}

impl FallingBytesParser {
    pub fn new() -> Result<FallingBytesParser, String> {
        regex::Regex::new(r"(\d+) *, *(\d+)")
            .map_err(error::regex_create)
            .map(|re|FallingBytesParser { byte_position_re: re })
    }

    fn parse_number(number: &str, line_num: usize, name: &str) -> Result<usize, String> {
        number.parse().map_err(|e|error::number_parse(line_num, name, e))
    }

    fn parse_line(&self, line: Line) -> Result<BytePosition, String> {
        let (x, y) = match self.byte_position_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [x, y])) => (x, y),
            None => return Err(error::line_parse(line.number)),
        };

        result::zip(
            Self::parse_number(x, line.number, "X"),
            Self::parse_number(y, line.number, "Y"),
            BytePosition::new,
        )
    }
}

impl Parse<FallingBytes> for FallingBytesParser {
    fn parse(&self, vec_line: VecLine) -> Result<FallingBytes, String> {
        collect(vec_line.lines.into_iter().map(|line|self.parse_line(line)).collect()).map(FallingBytes)
    }
}