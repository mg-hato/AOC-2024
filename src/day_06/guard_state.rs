use super::direction::Direction;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
/// A guard's state:
/// - current position as number-pair `(r,c)`: row `r`, column `c` (both 0-indexed)
/// - current facing direction
pub struct GuardState {
    pub position: (usize, usize),
    pub direction: Direction,
}

impl GuardState {
    pub fn new(position: (usize, usize), direction: Direction) -> GuardState {
        GuardState { position, direction }
    }

    /// Returns the guard's next position assuming there are no obstacles in front of the guard.
    pub fn next_position(&self) -> Option<(usize, usize)> {
        self.direction.next(self.position)
    }

    pub fn rotate(&self) -> GuardState {
        GuardState::new(self.position, self.direction.rotate())
    }

    /// Returns true if and only if the guard is facing the given position
    pub fn is_facing(&self, position: (usize, usize)) -> bool {
        let (guard_row, guard_col) = self.position;
        let (pos_row, pos_col) = position;

        match self.direction {
            Direction::Up => guard_col == pos_col && guard_row > pos_row,
            Direction::Right => guard_row == pos_row && guard_col < pos_col,
            Direction::Down => guard_col == pos_col && guard_row < pos_row,
            Direction::Left => guard_row == pos_row && guard_col > pos_col,
        }
    }
}