#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct UPosition {
    pub row: usize,
    pub col: usize,
}

impl UPosition {
    pub fn new(pos: (usize, usize)) -> UPosition {
        let (row, col) = pos;
        UPosition { row, col }
    }
}