use crate::{helper::{direction::Direction, result::collect, table::Table}, parser::Parse, reader::{Line, VecLine}};

use super::models::{Field, MapAndMoves, RobotMoves};

pub struct MapAndMovesParser;

mod error {
    const PREFIX: &str = "[Parser D-15]";

    pub fn unsupported_char(c: char, line_num: usize) -> String {
        format!("{} unsupported character '{}' on line #{}", PREFIX, c, line_num)
    }
}

impl MapAndMovesParser {
    fn is_empty(line: &Line) -> bool {
        line.text.trim().len() == 0
    }

    fn try_parse_map_row(line: &Line) -> Option<Vec<Field>> {
        if Self::is_empty(line) { return None; }
        let parsed = line.text.chars().map(|c|match c {
            '.' => Ok(Field::Empty),
            '#' => Ok(Field::Wall),
            '@' => Ok(Field::Robot),
            'O' => Ok(Field::Crate),
            _   => Err(()),
        }).collect();
        collect(parsed).map(Some).unwrap_or(None)
    }

    fn parse_map(lines: Vec<Line>) -> Result<(Table<Field>, Vec<Line>), String> {
        let mut i = 0;
        let mut parsed_rows = vec![];
        while i < lines.len() {
            if let Some(parsed_row) = Self::try_parse_map_row(&lines[i]) {
                parsed_rows.push(parsed_row);
                i += 1;
            }
            else { break; }
        }
        
        Table::new(parsed_rows).map(|table|(table, lines.into_iter().skip(i).collect()))
    }

    fn parse_moves(lines: Vec<Line>) -> Result<Vec<RobotMoves>, String> {
        let mut moves = vec![];
        for line in lines {
            if Self::is_empty(&line) { continue; }
            let parsed_moves = line.text.trim().chars().map(|c|match c {
                '^' => Ok(Direction::Up),
                'v' => Ok(Direction::Down),
                '>' => Ok(Direction::Right),
                '<' => Ok(Direction::Left),
                c   => Err(error::unsupported_char(c, line.number))
            }).collect();
            moves.push(collect(parsed_moves).map(RobotMoves));
        }
        collect(moves)
    }
}

impl Parse<MapAndMoves> for MapAndMovesParser {
    fn parse(&self, vec_line: VecLine) -> Result<MapAndMoves, String> {
        Self::parse_map(vec_line.lines)
            .and_then(|(map, lines)|Self::parse_moves(lines).map(|moves|MapAndMoves(map, moves)))
    }
}