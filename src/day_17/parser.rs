use crate::{helper::result::collect, parser::Parse, reader::{Line, VecLine}};

use super::model::ProgramInformation;


pub struct ProgramInformationParser {
    register_a_re: regex::Regex,
    register_b_re: regex::Regex,
    register_c_re: regex::Regex,
    program_re: regex::Regex,
}

mod error {
    use std::num::ParseIntError;

    const PREFIX: &str = "[Parser D-17]";

    pub fn create_regex(usecase: &str, e: regex::Error) -> String {
        format!("{} could not create regex for {}, because {}.", PREFIX, usecase, e)
    }

    pub fn not_four_lines(actual_length: usize) -> String {
        format!("{} parsing failed. Expected exactly 4 lines for parsing, received {}", PREFIX, actual_length)
    }
    
    pub fn parsing_line(line_num: usize, usecase: &str) -> String {
        format!("{} failed to parse line #{} for {}", PREFIX, line_num, usecase)
    }
    
    pub fn parsing_num(line_num: usize, number: &str, e: ParseIntError) -> String {
        format!("{} failed to parse number {} on line #{} due to {}", PREFIX, number, line_num, e)
    }
}

impl ProgramInformationParser {
    pub fn new() -> Result<ProgramInformationParser, String> {
        let a = regex::Regex::new(r"^Register +A *: *(\d+)$").map_err(|e|error::create_regex("register A", e));
        let b = regex::Regex::new(r"^Register +B *: *(\d+)$").map_err(|e|error::create_regex("register B", e));
        let c = regex::Regex::new(r"^Register +C *: *(\d+)$").map_err(|e|error::create_regex("register C", e));
        let program = regex::Regex::new(r"^Program *: *(\d+(?: *, *\d+)*)$")
            .map_err(|e|error::create_regex("program sequence", e));
        match collect(vec![a, b, c, program]) {
            Err(e) => Err(e),
            Ok(regexes) => Ok(ProgramInformationParser {
                register_a_re: regexes[0].clone(),
                register_b_re: regexes[1].clone(),
                register_c_re: regexes[2].clone(),
                program_re:    regexes[3].clone(),
            })
        }
    }

    fn parse_register(line: &Line, register_re: &regex::Regex, usecase: &str) -> Result<u64, String> {
        match register_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [value])) => {
                value.parse()
                    .map_err(|e|error::parsing_num(line.number, value, e))
            },
            None => Err(error::parsing_line(line.number, usecase)),
        }
    }

    fn parse_program_sequence(&self, line: &Line) -> Result<Vec<u64>, String> {
        let sequence = match self.program_re.captures(&line.text).map(|c|c.extract()) {
            None => return Err(error::parsing_line(line.number, "program sequence")),
            Some((_, [sequence])) => sequence,
        };

        let parsed_sequence = sequence.split(",")
            .map(|untrimmed_number|untrimmed_number.trim())
            .map(|number|number.parse().map_err(|e|error::parsing_num(line.number, number, e)))
            .collect();

        collect(parsed_sequence)
    }
}

impl Parse<ProgramInformation> for ProgramInformationParser {
    fn parse(&self, vec_line: VecLine) -> Result<ProgramInformation, String> {
        if vec_line.lines.len() != 4 {
            return Err(error::not_four_lines(vec_line.lines.len()));
        }

        // work out all registers, if any of it fails parsing, return error immediately
        let registers = match collect(vec![
            Self::parse_register(&vec_line.lines[0], &self.register_a_re, "register A"),
            Self::parse_register(&vec_line.lines[1], &self.register_b_re, "register B"),
            Self::parse_register(&vec_line.lines[2], &self.register_c_re, "register C"),
        ]) {
            Err(e) => return Err(e),
            Ok(registers) => registers,
        };

        // work out program sequence and create 
        self.parse_program_sequence(&vec_line.lines[3]).map(|program_sequence| ProgramInformation {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            program: program_sequence,
        })
    }
}