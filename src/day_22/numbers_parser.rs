use crate::{day_22::model::Numbers, helper::result::collect, parser::Parse, reader::{Line, VecLine}};


mod error {
    use std::num::ParseIntError;

    const PREFIX: &str = "[D-22 parser]";

    pub fn number_regex(err: regex::Error) -> String {
        format!("{} could not create parser due to regex error: {}", PREFIX, err)
    }

    pub fn line_match(line_num: usize) -> String {
        format!("{} line #{} does not match regex", PREFIX, line_num)
    }

    pub fn parse(err: ParseIntError, number: &str, line_num: usize) -> String {
        format!("{} line #{} failed to parse number '{}' due to parsing error {}", PREFIX, line_num, number, err)
    }
}

pub struct NumbersParser {
    number_re: regex::Regex
}

impl NumbersParser {
    pub fn new() -> Result<NumbersParser, String> {
        regex::Regex::new(r"^(\d+)$")
            .map(|number_re|NumbersParser { number_re })
            .map_err(error::number_regex)
    }

    fn parse_line(&self, line: Line) -> Result<u64, String> {
        match self.number_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [number_txt])) => number_txt.parse()
                .map_err(|err|error::parse(err, number_txt, line.number)),
            None => Err(error::line_match(line.number)),
        }
    }
}

impl Parse<Numbers> for NumbersParser {
    fn parse(&self, vec_line: VecLine) -> Result<Numbers, String> {
        collect(vec_line.lines.into_iter().map(|line|self.parse_line(line)).collect()).map(Numbers)
    }
}