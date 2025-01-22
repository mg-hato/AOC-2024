use crate::helper::{movement::Movement, position::UPosition};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
/// Represents the movement direction. Becase the read input goes from top to bottom (row-wise),
/// left to right (column-wise), going up-down is change in row, and left-right is change in column
pub enum Direction {
    /// Row decreases
    Up,
    /// Column increases
    Right,
    /// Row increases
    Down,
    /// Column decreases
    Left,
}

use self::Direction::*;

impl Direction {
    /// Given the current position, returns an option to the next position that corresponds
    /// to `Direction` of `self`. Because position is described as `(row, col)` where `row` and `col`
    /// are non-negative numbers, `None` will be returned when the next position would go out of bounds.
    /// (e.g. `Up.next` of position `(0, 10)`)
    pub fn next(&self, current: UPosition) -> Option<UPosition> {
        self.movement().apply(current)
    }

    pub fn all() -> Vec<Direction> {
        vec![Up, Right, Down, Left]
    }

    /// Returns the corresponding unit movement for the direction
    pub fn movement(&self) -> Movement {
        use crate::helper::movement::unit::*;
        match self {
            Up    => UP,
            Right => RIGHT,
            Down  => DOWN,
            Left  => LEFT,
        }
    }

    /// Returns direction after one rotation (clockwise)
    pub fn rotate(&self) -> Direction {
        match self {
            Up    => Right,
            Right => Down,
            Down  => Left,
            Left  => Up,
        }
    }
}