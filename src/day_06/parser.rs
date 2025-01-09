use crate::{helper::result::collect, parser::Parse, reader::Line};

use super::models::{LaboratoryMap, LaboratoryMapField};

pub struct LaboratoryMapParser {
    map_re: regex::Regex,
}

mod error {
    use crate::{helper::display::vector_display, reader::Line};

    const PREFIX: &str = "[Parser D-06]";
    pub fn regex_error(e: regex::Error) -> String {
        vector_display(&vec![
            format!("{} error while creating the parser.", PREFIX),
            format!("Could not create regex to match map's field due to error '{}'", e),
        ], " ")
    }

    pub fn line_does_not_match_regex(line: &Line) -> String {
        format!("{} line #{} does not match map regex.", PREFIX, line.number)
    }
}

impl LaboratoryMapParser {
    pub fn new() -> Result<LaboratoryMapParser, String> {
        regex::Regex::new(r"^[#.^]+$")
            .map(|map_re|LaboratoryMapParser{ map_re })
            .map_err(error::regex_error)
    }

    fn parse_line(&self, line: &Line) -> Result<Vec<LaboratoryMapField>, String> {
        if !self.map_re.is_match(&line.text) {
            Err(error::line_does_not_match_regex(line)) 
        } else {
            let map_row = line.text.chars().map(|c|match c {
                '#' => LaboratoryMapField::Block,
                '.' => LaboratoryMapField::Free,
                _   => LaboratoryMapField::Guard,
            }).collect();
            Ok(map_row)
        }
    }
}

impl Parse<LaboratoryMap> for LaboratoryMapParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<LaboratoryMap, String> {
        collect(vec_line.lines.into_iter()
            .map(|line|self.parse_line(&line))
            .collect())
            .map(LaboratoryMap::new)
    }
}