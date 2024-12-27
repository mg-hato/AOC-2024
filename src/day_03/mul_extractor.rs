use crate::{answer::{Answer, DisplayableAnswer}, helper::result::{collect, zip}, reader::{Line, VecLine}, solver::Solve};

use super::instruction::Instruction;

pub struct MulExtractor {
    mul_re: regex::Regex,
    conditional_detection: bool,
}

mod error {
    const PREFIX : &str = "[Solver D-03 P1]";
    
    pub fn regex_error(e: regex::Error) -> String {
        format!("{} regex creation failed because '{}'", PREFIX, e)
    }

    pub fn pattern_mismatch_error(whole_match: &str, line_num: usize) -> String {
        format!("{} fatal pattern mismatch happened when trying to resolve mul at line {} from '{}'",
            PREFIX, line_num, whole_match)
    }
    
    pub fn number_parse_error(e: std::num::ParseIntError, number: &str, whole_match: &str, line_num: usize) -> String {
        format!("{} failed to parse number '{}' from mul expression '{}' on line {} because of parsing error '{}'",
            PREFIX, number, whole_match, line_num, e)
    }

    pub fn overflow_error(acc: u32, x: u32, y: u32) -> String {
        format!("{} overflow occurred when attempting 'acc + mul(x,y)'. acc = {}, x = {}, y = {}", PREFIX, acc, x, y)
    }
}

impl MulExtractor {
    pub fn new(conditional_detection: bool) -> Result<MulExtractor, String> {
        regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)")
            .map(|mul_re|MulExtractor{ mul_re, conditional_detection })
            .map_err(error::regex_error)
    }

    fn resolve_num(number: &str, whole_match: &str, line_num: usize) -> Result<u32, String> {
        number.parse::<u32>().map_err(|e|error::number_parse_error(e, number, whole_match, line_num))
    }

    fn resolve_instruction(captures: regex::Captures, line_num: usize) -> Result<Instruction, String> {
        let whole_match = &captures[0];
        match whole_match {
            "do()" => Ok(Instruction::Do),
            "don't()" => Ok(Instruction::DoNot),
            mul if mul.starts_with("mul") && captures.get(1).is_some() && captures.get(2).is_some() => {
                let resolve = |number|Self::resolve_num(number, whole_match, line_num);
                zip(resolve(&captures[1]), resolve(&captures[2]), Instruction::Mul)
            }
            _ => Err(error::pattern_mismatch_error(whole_match, line_num))
        }
    }

    fn extract_instructions(&self, line: &Line) -> Vec<Result<Instruction, String>> {
        self.mul_re.captures_iter(&line.text())
            .map(|captures|Self::resolve_instruction(captures, line.number()))
            .collect()
    }

    fn safe_mul_add(acc: u32, x: u32, y: u32) -> Result<u32, String> {
        x.checked_mul(y)
            .and_then(|product|acc.checked_add(product))
            .ok_or_else(||error::overflow_error(acc, x, y))
    }

    fn process_instruction(&self, acc: (u32, bool), inst: Instruction) -> Result<(u32, bool), String> {
        let (accumulator, ignore) = acc;
        match inst {
            Instruction::Mul(x,y) if !ignore => Self::safe_mul_add(accumulator, x, y).map(|value|(value, ignore)),
            Instruction::Do if self.conditional_detection => Ok((accumulator, false)),
            Instruction::DoNot if self.conditional_detection => Ok((accumulator, true)),
            _ => Ok(acc),
        }
    }
}

impl Solve<VecLine> for MulExtractor {
    fn solve(&self, input: VecLine) -> Result<Answer, String> {
        collect(input.lines.iter().flat_map(|line|self.extract_instructions(line)).collect())
            ?.into_iter()
            .try_fold((0u32, false), |acc,inst|self.process_instruction(acc, inst))
            .map(|(acc,_)|DisplayableAnswer::new(acc))
    }
}