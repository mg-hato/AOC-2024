

use crate::{helper::{re::get_captures, result}, parser::Parse, reader::{Line, VecLine}};

use super::models::{NumberPair, NumberPairList};

pub struct NumberPairListParser {
    re: regex::Regex
}


mod error {
    const PREFIX : &str = "[Parser D-01]";

    pub fn regex_error(e: regex::Error) -> String {
        format!("{} could not compile line regex {}", PREFIX, e)
    }

    pub fn parse_num_error(number_str: &String, is_first_num: bool, e: std::num::ParseIntError, line_num: usize) -> String {
        let name = if is_first_num { "1st number" } else { "2nd number" };
        format!("{} could not parse {} '{}' on line #{} because parsing error occurred '{}'",
            PREFIX, name, number_str, line_num, e)
    }

    pub fn parse_line_error(line_num: usize) -> String {
        format!("{} line #{} failed to match line-regex", PREFIX, line_num)
    }
}

impl NumberPairListParser {
    pub fn new() -> Result<NumberPairListParser, String> {
        regex::Regex::new(r"^(\d+) +(\d+)$")
            .map_err(error::regex_error)
            .map(|re|NumberPairListParser{re})
    }

    fn parse_num(number_str: &String, is_first_num: bool, line_num: usize) -> Result<u32, String> {
        number_str.parse::<u32>()
            .map_err(|e|error::parse_num_error(number_str, is_first_num, e, line_num))
    }

    fn parse_number_pair(fst: &String, snd: &String, line_num: usize) -> Result<NumberPair, String> {
        let fst = Self::parse_num(fst, true, line_num);
        let snd = Self::parse_num(snd, false, line_num);
        result::zip(fst, snd, NumberPair)
    }
    
    fn parse_single_line(&self, line: &Line) -> Result<NumberPair, String>
    {
        match get_captures(&self.re, &line.textf()) {
            Some(v) if v.len() == 3 => Self::parse_number_pair(&v[1], &v[2], line.numberf()),
            _ => Err(error::parse_line_error(line.numberf())),
        }
    }
}


impl Parse<NumberPairList> for NumberPairListParser {
    fn parse(&self, vec_line: VecLine) -> Result<NumberPairList, String> {
        let vector_of_results = vec_line.lines.iter()
            .map(|line|self.parse_single_line(line))
            .collect();
        result::collect(vector_of_results).map(NumberPairList::new)
    }
}