use crate::{helper::result::collect, parser::Parse, reader::{Line, VecLine}};

use super::model::Codes;


pub struct CodeParser {
    pub code_re: regex::Regex
}

mod error {
    const PREFIX: &str = "[Parser D-21]";

    pub fn regex_create(purpose: &str, e: regex::Error) -> String {
        format!("{} could not create regex for {} due to: {}", PREFIX, purpose, e)
    }

    pub fn parse_line(line_num: usize) -> String {
        format!("{} could not parse line #{}", PREFIX, line_num)
    }
}

impl CodeParser {
    pub fn new() -> Result<CodeParser, String> {
        regex::Regex::new(r"^([0-9A]+)$")
            .map_err(|e|error::regex_create("codes", e))
            .map(|re|CodeParser { code_re: re })
    }

    fn parse_line(&self, line: &Line) -> Result<String, String> {
        match self.code_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [code])) => Ok(code.to_string()),
            None => Err(error::parse_line(line.number)),
        }
    }
}

impl Parse<Codes> for CodeParser {
    fn parse(&self, vec_line: VecLine) -> Result<Codes, String> {
        collect(vec_line.lines.iter().map(|line|self.parse_line(line)).collect())
            .map(Codes)
    }
}