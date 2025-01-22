use crate::helper::{direction::Direction, movement::{Delta, Movement}, position::UPosition};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
/// A guard's state:
/// - current position as number-pair `(r,c)`: row `r`, column `c` (both 0-indexed)
/// - current facing direction
pub struct GuardState {
    pub position: UPosition,
    pub direction: Direction,
}

impl GuardState {
    pub fn new(position: UPosition, direction: Direction) -> GuardState {
        GuardState { position, direction }
    }

    /// Returns unit movement corresponding to current facing direction
    pub fn movement(&self) -> Movement {
        self.direction.movement()
    }

    /// Returns guard state after performing directional rotation
    pub fn rotate(&self) -> GuardState {
        GuardState::new(self.position, self.direction.rotate())
    }


    /// Returns true iff two delta changes are "matching":
    /// - both are zero deltas
    /// - both are NOT zero deltas and are of the same delta-direction
    fn delta_match(lhs: Delta, rhs: Delta) -> bool {
        (lhs.is_zero() == rhs.is_zero()) // both are zeros or both are not zeros
            && (lhs.is_zero() || lhs.is_same_dir(rhs)) // and they are zeros or they are same delta direction
    }

    /// Returns true if and only if the guard is facing the given position
    pub fn is_facing(&self, position: UPosition) -> bool {
        let Movement { row: row_diff, col: col_diff } = Movement::infer(self.position, position);
        let Movement { row: row_dir, col: col_dir} = self.movement();
        Self::delta_match(row_diff, row_dir) && Self::delta_match(col_diff, col_dir)
    }
}