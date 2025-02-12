use std::fmt::Display;

use crate::helper::{direction::Direction, display::vector_display, table::Table};


#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Field {
    Empty,
    Wall,
    Crate,
    Robot,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Field::Empty => '.',
            Field::Wall => '#',
            Field::Crate => 'O',
            Field::Robot => '@',
        })
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RobotMoves(pub Vec<Direction>);

impl Display for RobotMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let RobotMoves(directions) = self;
        write!(f, "[{}]", vector_display(&directions.iter().map(|dir|match dir {
            Direction::Up    => '^',
            Direction::Right => '>',
            Direction::Down  => 'v',
            Direction::Left  => '<',
        }).collect(), ""))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MapAndMoves(pub Table<Field>, pub Vec<RobotMoves>);

impl Display for MapAndMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let MapAndMoves(map, moves) = self;
        write!(f, "{{Map:{};Moves{}}}", map, vector_display(moves, ","))
    }
}