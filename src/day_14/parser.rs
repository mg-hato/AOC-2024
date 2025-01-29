use crate::{day_14::models::XY, helper::result::collect, parser::Parse, reader::{Line, VecLine}};

use super::models::{Robot, RobotList};



mod error {
    use std::num::ParseIntError;

    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Parser D-14]";

    pub fn regex_creation(e: regex::Error) -> String {
        format!("{} could not create line regex for parser due to regex error '{}'", PREFIX, e)
    }

    pub fn line_not_match(line_num: usize) -> String {
        vector_display(&vec![
            format!("{} could not parse line #{}", PREFIX, line_num),
            format!("because it does not match the expected regular expression.")
        ], " ")
    }

    pub fn number_parse(number: &str, line_num: usize, e: ParseIntError) -> String {
        vector_display(&vec![
            format!("{} failed to parse number '{}' on line #{}", PREFIX, line_num, number),
            format!("due to parsing error '{}'", e),
        ], " ")
    }
}

pub struct RobotListParser {
    line_re: regex::Regex,
}

impl RobotListParser {
    pub fn new() -> Result<RobotListParser, String> {
        regex::Regex::new(r"p *= *(\d+) *, *(\d+) *v *= *(-?\d+) *, *(-?\d+)")
            .map_err(error::regex_creation)
            .map(|line_re|RobotListParser { line_re })
    }

    fn parse_line(&self, line: Line) -> Result<Robot, String> {
        if let Some((_, [px, py, vx, vy])) = self.line_re.captures(&line.text).map(|c|c.extract()) {
            collect([px, py, vx, vy].iter()
                .map(|&number|number.parse::<i32>().map_err(|e|error::number_parse(number, line.number, e)))
                .collect())
                .map(|numbers|Robot{
                    position: XY { x: numbers[0], y: numbers[1] },
                    velocity: XY { x: numbers[2], y: numbers[3] },
                })
        } else {
            Err(error::line_not_match(line.number))
        }
    }
}

impl Parse<RobotList> for RobotListParser {
    fn parse(&self, vec_line: VecLine) -> Result<RobotList, String> {
        collect(vec_line.lines.into_iter().map(|line|self.parse_line(line)).collect())
            .map(RobotList)
    }
}