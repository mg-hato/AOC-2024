use crate::helper::{movement::{self, Movement}, position::UPosition};

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

impl Direction {
    /// Given the current position, returns an option to the next position that corresponds
    /// to `Direction` of `self`. Because position is described as `(row, col)` where `row` and `col`
    /// are non-negative numbers, `None` will be returned when the next position would go out of bounds.
    /// (e.g. `Up.next` of position `(0, 10)`)
    pub fn next(&self, current: UPosition) -> Option<UPosition> {
        self.movement().apply(current)
    }

    /// Returns the corresponding unit movement for the direction
    pub fn movement(&self) -> Movement {
        match self {
            Direction::Up    => movement::unit::UP,
            Direction::Right => movement::unit::RIGHT,
            Direction::Down  => movement::unit::DOWN,
            Direction::Left  => movement::unit::LEFT,
        }
    }

    /// Returns direction after one rotation (clockwise)
    pub fn rotate(&self) -> Direction {
        match self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        }
    }
}