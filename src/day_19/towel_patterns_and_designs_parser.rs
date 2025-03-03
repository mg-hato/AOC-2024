
use crate::{parser::Parse, reader::Line};

use super::model::TowelPatternsAndDesigns;

mod error {
    const PREFIX: &str = "[Parser D-19]";

    pub fn regex_create(purpose: &str, e: regex::Error) -> String {
        format!("{} could not create regex for {} because of the error: {}", PREFIX, purpose, e)
    }

    pub fn invalid_towel_pattern(index: usize, towel: &str) -> String {
        format!("{} invalid towel pattern at index {}: '{}'", PREFIX, index, towel)
    }

    pub fn invalid_towel_design(line_num: usize, towel: &String) -> String {
        format!("{} invalid towel design on line #{}: '{}'", PREFIX, line_num, towel)
    }
}

pub struct TowelPatternsAndDesignsParser {
    // patterns_re: regex::Regex,
    towel_re: regex::Regex,
}

impl TowelPatternsAndDesignsParser {
    pub fn new() -> Result<TowelPatternsAndDesignsParser, String> {
        regex::Regex::new(r"^[wubrg]+$").map_err(|e|error::regex_create("towel", e))
            .map(|re|TowelPatternsAndDesignsParser { towel_re: re })
    }

    fn parse_patterns(&self, line: Line) -> Result<Vec<String>, String> {
        let mut parsed_patterns = vec![];
        for (index, pattern) in line.text.split(",").map(|bit|bit.trim()).enumerate() {
            if !self.towel_re.is_match(pattern) {
                return Err(error::invalid_towel_pattern(index, pattern))
            }
            parsed_patterns.push(pattern.to_string());
        }
        Ok(parsed_patterns)
    }

    fn parse_design(&self, line: Line) -> Result<String, String> {
        if self.towel_re.is_match(&line.text) {
            Ok(line.text)
        } else {
            Err(error::invalid_towel_design(line.number, &line.text))
        }
    }
}

impl Parse<TowelPatternsAndDesigns> for TowelPatternsAndDesignsParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<TowelPatternsAndDesigns, String> {
        let mut patterns = vec![];
        let mut designs = vec![];
        for (i, line) in vec_line.lines.into_iter().enumerate() {
            if i == 0 {
                patterns = match self.parse_patterns(line) {
                    Ok(patterns) => patterns,
                    Err(e) => return Err(e),    
                };
            } else {
                match self.parse_design(line) {
                    Ok(design) => designs.push(design),
                    Err(e) => return Err(e),
                };
            }
        }

        Ok(TowelPatternsAndDesigns { patterns, designs })
    }
}