/// A idea of position movement
pub struct Movement(Change, Change);

impl Movement {
    /// Infer a movement between two given position to go `from` into `to`
    pub fn infer(from: (usize, usize), to: (usize, usize)) -> Movement {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        Movement(Change::infer(from_row, to_row), Change::infer(from_col, to_col))
    }

    /// Apply the movement to produce a new position
    pub fn apply(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = pos;
        let Movement(row_change, col_change) = self;
        match (row_change.apply(row), col_change.apply(col)) {
            (Some(new_row), Some(new_col)) => Some((new_row, new_col)),
            _ => None
        }
    }
}

/// A change in number: increase by or decrease by
enum Change {
    Inc(usize),
    Dec(usize),
}

impl Change {
    /// Infer a change that happens between to go `from` -> `to`
    pub fn infer(from: usize, to: usize) -> Change {
        let absolute_change = from.abs_diff(to);
        if from <= to {
            Change::Inc(absolute_change)
        } else {
            Change::Dec(absolute_change)
        }
    }

    /// Apply the change onto a value
    pub fn apply(&self, value: usize) -> Option<usize> {
        match *self {
            Change::Inc(x) => value.checked_add(x),
            Change::Dec(x) => value.checked_sub(x),
        }
    }
}