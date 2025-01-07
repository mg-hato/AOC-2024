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
    pub fn next(&self, current: (usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = current;
        match self {
            Direction::Up if row > 0   => Some((row - 1, col    )),
            Direction::Down            => Some((row + 1, col    )),
            Direction::Left if col > 0 => Some((row    , col - 1)),
            Direction::Right           => Some((row    , col + 1)),
            _                          => None
        }
    }

    pub fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}