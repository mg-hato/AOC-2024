use std::vec;

use crate::{day_25::model::{KeyLockSchema, KeyLockSpace, KeyLockSchematics}, helper::table::Table, parser::Parse, reader::Line};


mod error {
    const PREFIX: &str = "[D-25 parser]";
    pub fn unexpected_chars(line_num: usize) -> String {
        format!("{} line #{} contains unsupported character(s)", PREFIX, line_num)
    }

    pub fn schema(line_num: usize, err: String) -> String {
        format!("{} could not create a schema ending with line #{} due to error {}", PREFIX, line_num, err)
    }
}
pub struct SchematicsParser;

impl SchematicsParser {
    pub fn new() -> SchematicsParser {
        SchematicsParser
    }

    fn parse_line(line: &Line) -> Result<Vec<KeyLockSpace>, String> {
        if line.text.chars().all(|c|c == '.' || c == '#') {
            Ok(line.text.chars()
                .map(|c|if c == '.' { KeyLockSpace::Space } else { KeyLockSpace::Block })
                .collect())
        } else {
            Err(error::unexpected_chars(line.number))
        }
    }

    fn add_table(
        table: Vec<Vec<KeyLockSpace>>,
        schematics: Result<Vec<KeyLockSchema>, String>,
    ) -> Result<Vec<KeyLockSchema>, String>
        {
            if schematics.is_err() || table.is_empty() { return schematics; }

            let mut schematics = schematics.unwrap();
            match Table::new(table) {
                Ok(schema) => schematics.push(KeyLockSchema(schema)),
                Err(message) => return Err(error::schema(0, message)),
            }
            Ok(schematics)
        }
}

impl Parse<KeyLockSchematics> for SchematicsParser {
    fn parse(&self, vec_line: crate::reader::VecLine) -> Result<KeyLockSchematics, String> {
        let mut schematics = Ok(vec![]);
        let mut current_table = vec![];
        for line in vec_line.lines {
            let current_line = match Self::parse_line(&line) {
                Ok(current_line) => current_line,
                Err(message) => return Err(message),
            };
            if !current_line.is_empty() {
                current_table.push(current_line);
            } else if !current_table.is_empty() {
                schematics = Self::add_table(current_table, schematics);
                current_table = vec![];
            }
        }

        Self::add_table(current_table, schematics).map(KeyLockSchematics)
    }
}