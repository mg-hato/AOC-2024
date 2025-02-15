use std::collections::HashMap;

use crate::{helper::{result::collect, table::Table}, parser::Parse, reader::{Line, VecLine}};

use super::model::Field;

pub mod error {
    const PREFIX: &str = "[Parser D-16]";

    pub fn unsupported_char(c: char, line_num: usize) -> String {
        format!("{} unsupported character '{}' encountered on line #{}.", PREFIX, c, line_num)
    }
}

pub struct ReindeerMazeParser {
    char_mapping: HashMap<char, Field>,
}

impl ReindeerMazeParser {
    pub fn new() -> ReindeerMazeParser {
        ReindeerMazeParser { char_mapping: HashMap::from([
            ('S', Field::Start),
            ('E', Field::End),
            ('.', Field::Empty),
            ('#', Field::Wall),
        ]) }
    }
    
    fn parse_line(&self, line: Line) -> Result<Vec<Field>, String> {
        let parsed_fields = line.text.chars().map(|c|
            if let Some(&field) = self.char_mapping.get(&c) { Ok(field) }
            else { Err(error::unsupported_char(c, line.number)) } 
        ).collect();
        collect(parsed_fields)
    }
}


impl Parse<Table<Field>> for ReindeerMazeParser {
    fn parse(&self, vec_line: VecLine) -> Result<Table<Field>, String> {
        let rows = vec_line.lines.into_iter()
            .map(|line|self.parse_line(line))
            .collect();

        collect(rows).and_then(Table::new)
    }
}