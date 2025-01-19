use crate::{helper::{result::collect, table::Table}, parser::Parse, reader::Line};


pub struct TopographicMapParser;

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Parser D-10]";
    
    pub fn unsupported_character_error(c: char, line_num: usize) -> String {
        vector_display(&vec![
            format!("{} error while parsing topographic map on line #{}", PREFIX, line_num),
            format!("character '{}' is not supported, a digit was expected", c),
        ], " ")
    }
}

impl TopographicMapParser {
    fn parse_character(c: char, line_num: usize) -> Result<usize, String> {
        c.to_digit(10)
            .map(|digit|digit as usize)
            .ok_or_else(||error::unsupported_character_error(c, line_num))
    }

    fn parse_line(line: Line) -> Result<Vec<usize>, String> {
        collect(line.text.chars().map(|c|Self::parse_character(c, line.number)).collect())
    }
}

impl Parse<Table<usize>> for TopographicMapParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<Table<usize>, String> {
        collect(vec_line.lines.into_iter().map(Self::parse_line).collect()).and_then(Table::new)
    }
}