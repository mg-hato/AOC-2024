use crate::{helper::{result::collect, table::Table}, parser::Parse, reader::Line};


mod error {
    const PREFIX: &str = "[Parser D-12]";
    pub fn unsupported_character(c: char, line_num: usize) -> String {
        format!("{} unsupported character '{}' found on line #{}", PREFIX, c, line_num)
    }
}

pub struct GardenParser;

impl GardenParser {
    fn parse_line(line: Line) -> Result<Vec<char>, String> {
        let checked_characters = line.text.chars().into_iter().map(|c|match c.is_ascii_uppercase() {
            true  => Ok(c),
            false => Err(error::unsupported_character(c, line.number))
        }).collect();
        collect(checked_characters)
    }
}

impl Parse<Table<char>> for GardenParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<Table<char>, String> {
        let parsed_lines = vec_line.lines.into_iter().map(Self::parse_line).collect();
        collect(parsed_lines).and_then(Table::new)
    }
}