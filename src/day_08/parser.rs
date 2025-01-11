use crate::{helper::{result::collect, table::Table}, parser::Parse, reader::Line};

use super::model::AntennaMapField;


mod error {
    const PREFIX: &str = "[Parser D-08]";

    pub fn unsupported_character_error(c: char, line_num: usize) -> String {
        format!("{} error while parsing line #{} unsupported character '{}'", PREFIX, line_num, c)
    }
}
pub struct AntennaMapParser;

impl AntennaMapParser {
    fn convert_character(ch: char, line_num: usize) -> Result<AntennaMapField, String> {
        match ch {
            '.' => Ok(AntennaMapField::Free),
            lower_letter if 'a' <= lower_letter && lower_letter <= 'z' => Ok(AntennaMapField::Antenna(lower_letter)),
            upper_letter if 'A' <= upper_letter && upper_letter <= 'Z' => Ok(AntennaMapField::Antenna(upper_letter)),
            digit if '0' <= digit && digit <= '9' => Ok(AntennaMapField::Antenna(digit)),
            _ => Err(error::unsupported_character_error(ch, line_num))
        }
    }

    fn parse_line(line: Line) -> Result<Vec<AntennaMapField>, String> {
        collect(line.text.chars().map(|c|Self::convert_character(c, line.number)).collect())
    }
}

impl Parse<Table<AntennaMapField>> for AntennaMapParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<Table<AntennaMapField>, String> {
        collect(vec_line.lines.into_iter().map(Self::parse_line).collect())
            .and_then(Table::new)
    }
}