use super::position::UPosition;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Movement {
    pub row: Delta,
    pub col: Delta,
}

impl Movement {
    pub fn new(row: Delta, col: Delta) -> Movement {
        Movement { row, col }
    }

    pub fn zero() -> Movement { Movement::new(Delta::zero(), Delta::zero()) }

    pub fn infer(from: UPosition, to: UPosition) -> Movement {
        let UPosition { row: fr, col: fc} = from;
        let UPosition { row: tr, col: tc} = to;
        Movement::new(Delta::infer(fr, tr), Delta::infer(fc, tc))
    }

    pub fn apply(&self, pos: UPosition) -> Option<UPosition> {
        let UPosition{ row: r, col: c } = pos;
        let Movement { row: row_change, col: col_change } = *self;
        match (row_change.apply(r), col_change.apply(c)) {
            (Some(new_r), Some(new_c)) => Some(UPosition::new((new_r, new_c))),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Movement {
        Movement { row: self.row.inverse(), col: self.col.inverse() }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Delta {
    Inc(usize),
    Dec(usize),
}

impl Delta {
    pub fn zero() -> Delta { Delta::Inc(0) }

    pub fn infer(from: usize, to: usize) -> Delta {
        let abs = from.abs_diff(to);
        if from <= to { Delta::Inc(abs) } else { Delta::Dec(abs) }
    }

    pub fn apply(&self, value: usize) -> Option<usize> {
        match *self {
            Delta::Inc(x) => value.checked_add(x),
            Delta::Dec(x) => value.checked_sub(x),
        }
    }

    /// Returns true if both `self` and `other` are of same delta-direction, i.e. both are `Inc` or both are `Dec`
    pub fn is_same_dir(&self, other: Delta) -> bool {
        match (self, other) {
            (Delta::Inc(_), Delta::Inc(_)) => true,
            (Delta::Dec(_), Delta::Dec(_)) => true,
            _ => false
        }
    }

    /// Returns the absolute numeric change
    pub fn get_absolute_change(&self) -> usize {
        match *self {
            Delta::Inc(x) => x,
            Delta::Dec(x) => x,
        }
    }

    /// Returns true if it is a zero-delta i.e. `Inc(0)` or `Dec(0)`
    pub fn is_zero(&self) -> bool {
        self.get_absolute_change() == 0
    }

    pub fn inverse(&self) -> Delta {
        match *self {
            Delta::Inc(x) => Delta::Dec(x),
            Delta::Dec(x) => Delta::Inc(x),
        }
    }
}

#[allow(dead_code)]
pub mod unit {
    use super::{Movement, Delta::{Dec, Inc}};

    /// Zero movement
    pub const ZERO: Movement = Movement { row: Inc(0), col: Inc(0) };

    /// Decreases row only by 1
    pub const UP: Movement = Movement { row: Dec(1), col: Inc(0) };

    /// Increases row only by 1
    pub const DOWN: Movement = Movement { row: Inc(1), col: Inc(0) };

    /// Decreases column only by 1
    pub const LEFT: Movement = Movement { row: Inc(0), col: Dec(1) };

    /// Increases column only by 1
    pub const RIGHT: Movement = Movement { row: Inc(0), col: Inc(1) };

    /// Decreases row by 1, decreases column by 1
    pub const UP_LEFT: Movement = Movement { row: Dec(1), col: Dec(1) };

    /// Decreases row by 1, increases column by 1
    pub const UP_RIGHT: Movement = Movement { row: Dec(1), col: Inc(1) };

    /// Increases row by 1, decreases column by 1
    pub const DOWN_LEFT: Movement = Movement { row: Inc(1), col: Dec(1) };

    /// Increases row by 1, increases column by 1
    pub const DOWN_RIGHT: Movement = Movement { row: Inc(1), col: Inc(1) };

    /// All 8 unit directions
    pub fn all() -> Vec<Movement> {
        vec![UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT]
    }

    /// All 4 unit partial directions: up, right, down, left
    pub fn all_partial() -> Vec<Movement> {
        vec![UP, RIGHT, DOWN, LEFT]
    }
}