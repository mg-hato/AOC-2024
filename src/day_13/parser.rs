use std::{num::ParseIntError, vec};

use crate::{helper::{position::UPosition, result::{self, collect}}, parser::Parse, reader::{Line, VecLine}};

use super::model::{ClawMachine, ClawMachines};


pub struct ClawMachinesParser {
    button_a_re: regex::Regex,
    button_b_re: regex::Regex,
    prize_re: regex::Regex,
}

mod error {
    use std::num::ParseIntError;

    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Parser D-13]";

    pub fn create_regex(purpose: &str, e: regex::Error) -> String {
        vector_display(&vec![
            format!("{} could not create parser.", PREFIX),
            format!("Failed to create regex for {} due to regex creation error '{}'", purpose, e),
        ], " ")
    }

    pub fn incorrect_line_count(line_count: usize) -> String {
        vector_display(&vec![
            format!("{} parsing aborted. The input lines are to be grouped into disjoint groups of 3 lines.", PREFIX),
            format!("However, the number of valid lines for parsing is {}", line_count),
        ], " ")
    }

    pub fn parse_failure(line_num: usize, target: &str) -> String {
        vector_display(&vec![
            format!("{} failed to parse line #{}.", PREFIX, line_num),
            format!("The line could not be interpreted as {}.", target),
        ], " ")
    }

    pub fn parse_button_a(line_num: usize) -> String {
        parse_failure(line_num, "button A's movement")
    }

    pub fn parse_button_b(line_num: usize) -> String {
        parse_failure(line_num, "button B's movement")
    }

    pub fn parse_prize(line_num: usize) -> String {
        parse_failure(line_num, "prize coordinates")
    }

    pub fn parse_number(number: &str, line_num: usize, e: ParseIntError) -> String {
        vector_display(&vec![
            format!("{} failed to parse number '{}' on line #{}", PREFIX, number, line_num),
            format!("because of the following '{}'.", e),
        ], " ")
    }
}

impl ClawMachinesParser {
    pub fn new() -> Result<ClawMachinesParser, String> {
        let button_a_re = regex::Regex::new(r"Button +A *: *X\+(\d+) *, *Y\+(\d+)")
            .map_err(|e|error::create_regex("button A", e));

        let button_b_re = regex::Regex::new(r"Button +B *: *X\+(\d+) *, *Y\+(\d+)")
            .map_err(|e|error::create_regex("button B", e));

        let prize_re = regex::Regex::new(r"Prize *: *X=(\d+) *, *Y=(\d+)")
            .map_err(|e|error::create_regex("prize", e));

        if button_a_re.is_ok() && button_b_re.is_ok() && prize_re.is_ok() {
            Ok(ClawMachinesParser {
                button_a_re: button_a_re.unwrap(),
                button_b_re: button_b_re.unwrap(),
                prize_re: prize_re.unwrap(),
            })
        } else {
            Err(collect(vec![button_a_re, button_b_re, prize_re]).unwrap_err())
        }
    }

    fn parse_number<ERRF>(number: &str, err_fn: ERRF) -> Result<usize, String>
    where ERRF: Fn(ParseIntError) -> String {
        number.parse().map_err(err_fn)
    }

    fn parse_uposition<ERRF>(line: &Line, re: &regex::Regex, err_fn: ERRF) -> Result<UPosition, String>
    where ERRF: Fn(usize) -> String {
        if let Some((_, [x, y])) = re.captures(&line.text).map(|c|c.extract()) {
            result::zip(
                Self::parse_number(x, |e|error::parse_number(x, line.number, e)),
                Self::parse_number(y, |e|error::parse_number(y, line.number, e)),
                |x, y|UPosition::new((x, y))
            )
        } else {
            Err((err_fn)(line.number))
        }
    }

    fn parse_claw_machine(&self, lines: &[&Line; 3]) -> Result<ClawMachine, String> {
        let button_a = Self::parse_uposition(&lines[0], &self.button_a_re, error::parse_button_a);
        let button_b = Self::parse_uposition(&lines[1], &self.button_b_re, error::parse_button_b);
        let prize = Self::parse_uposition(&lines[2], &self.prize_re, error::parse_prize);
        match collect(vec![button_a, button_b, prize]) {
            Ok(vector) => Ok(ClawMachine {
                button_a: vector[0],
                button_b: vector[1],
                prize: vector[2],
            }),
            Err(e) => Err(e),
        }
    }
}

impl Parse<ClawMachines> for ClawMachinesParser {
    fn parse(&self, vec_line: VecLine) -> Result<ClawMachines, String> {
        let lines = vec_line.lines;

        // If number of lines clearly indicates a problem at some point, raise it immediately
        if lines.len() % 3 != 0 { return Err(error::incorrect_line_count(lines.len())); }

        let mut parsed_results = vec![];
        let mut i = 0;
        while i < lines.len() / 3 {
            let parsed = self.parse_claw_machine(&[&lines[3 * i], &lines[3 * i + 1], &lines[3 * i + 2]]);
            parsed_results.push(parsed);
            i += 1;
        }
        collect(parsed_results).map(ClawMachines)
    }
}