use crate::{helper::result::{collect, zip}, parser::Parse, reader::Line};

use super::equation::{Equation, EquationList};


pub struct EquationListParser {
    equation_re: regex::Regex,
}

mod error {
    const PREFIX: &str = "[Parser D-07]";

    pub fn equation_regex_error(e: regex::Error) -> String {
        format!("{} could not create regex for equations due to following error '{}'", PREFIX, e)
    }

    pub fn paring_error(num: &str, line_num: usize, e: std::num::ParseIntError) -> String {
        format!("{} could not parse number '{}' on line #{} error '{}'", PREFIX, num, line_num, e)
    }

    pub fn pattern_not_matched(line_num: usize) -> String {
        format!("{} line #{} does not match the expected pattern", PREFIX, line_num)
    }
}

impl EquationListParser {
    pub fn new() -> Result<EquationListParser, String> {
        regex::Regex::new(r"^(\d+):((?: *\d+)+)$")
            .map_err(error::equation_regex_error)
            .map(|equation_re|EquationListParser { equation_re })
    }

    fn parse_single_number(number: &str, line_num: usize) -> Result<u64, String> {
        number.parse().map_err(|e|error::paring_error(number, line_num, e))
    }

    fn parse_rights(numbers: &str, line_num: usize) -> Result<Vec<u64>, String> {
        let parsed_results = numbers.split(' ')
            .filter(|s|!s.is_empty())
            .map(|number|Self::parse_single_number(number, line_num)).collect();
        collect(parsed_results)
    }

    fn parse_line(&self, line: Line) -> Result<Equation, String> {
        match self.equation_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [left, rights])) => {
                let left_value = Self::parse_single_number(left, line.number);
                let right_values = Self::parse_rights(rights, line.number);
                zip(left_value, right_values, Equation::new)
            }
            None => Err(error::pattern_not_matched(line.number)),
        }
    }
}

impl Parse<EquationList> for EquationListParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<EquationList, String> {
        let equation_results = vec_line.lines.into_iter().map(|line|self.parse_line(line)).collect();
        collect(equation_results).map(EquationList)
    }
}