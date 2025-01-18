use super::position::UPosition;

#[derive(Clone, Copy)]
pub struct Movement(Delta, Delta);

impl Movement {
    pub fn infer(from: UPosition, to: UPosition) -> Movement {
        let UPosition { row: fr, col: fc} = from;
        let UPosition { row: tr, col: tc} = to;
        Movement(Delta::infer(fr, tr), Delta::infer(fc, tc))
    }

    pub fn apply(&self, pos: UPosition) -> Option<UPosition> {
        let UPosition{ row: r, col: c } = pos;
        let Movement(row_change, col_change) = *self;
        match (row_change.apply(r), col_change.apply(c)) {
            (Some(new_r), Some(new_c)) => Some(UPosition::new((new_r, new_c))),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Movement {
        let Movement(r, c) = *self;
        Movement(r.inverse(), c.inverse())
    }
}

#[derive(Clone, Copy)]
enum Delta {
    Inc(usize),
    Dec(usize),
}

impl Delta {
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

    pub fn inverse(&self) -> Delta {
        match *self {
            Delta::Inc(x) => Delta::Dec(x),
            Delta::Dec(x) => Delta::Inc(x),
        }
    }
}