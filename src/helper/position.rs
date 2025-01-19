use std::fmt::Display;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct UPosition {
    pub row: usize,
    pub col: usize,
}

impl UPosition {
    pub fn new(pos: (usize, usize)) -> UPosition {
        let (row, col) = pos;
        UPosition { row, col }
    }

    pub fn zero() -> UPosition { UPosition::new((0, 0)) }

    pub fn pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl Display for UPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let UPosition { row, col } = *self;
        write!(f, "({},{})", row, col)
    }
}