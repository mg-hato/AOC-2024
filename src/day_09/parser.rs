use crate::{helper::result::collect, parser::Parse, reader::VecLine};

use super::model::DiskMap;

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Parser D-09]";

    pub fn input_shape_error(length: usize) -> String {
        vector_display(&vec![
            format!("{} error while parsing disk map", PREFIX),
            format!("exactly one line of input is expected"),
            format!("but there are {} lines", length),
        ], " ")
    }

    pub fn unsupported_character_error(c: char) -> String {
        vector_display(&vec![
            format!("{} error while parsing disk map", PREFIX),
            format!("character '{}' is not supported, a digit was expected", c),
        ], " ")
    }
}

pub struct DiskMapParser;

impl DiskMapParser {
    fn parse_character(c: char) -> Result<usize, String> {
        c.to_digit(10)
            .map(|digit|digit as usize)
            .ok_or_else(||error::unsupported_character_error(c))
    }
}

impl Parse<DiskMap> for DiskMapParser {
    fn parse(&self, vec_line: VecLine) -> Result<DiskMap, String> {
        if vec_line.lines.len() != 1 {
            Err(error::input_shape_error(vec_line.lines.len()))
        } else {
            collect(vec_line.lines[0].text.chars().map(Self::parse_character).collect()).map(DiskMap)
        }
    }
}