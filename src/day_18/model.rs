use std::fmt::Display;

use crate::helper::display::vector_display;


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BytePosition { pub x: usize, pub y: usize }

impl BytePosition {
    pub fn new(x: usize, y: usize) -> BytePosition { BytePosition { x, y } }
}

impl Display for BytePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(X:{},Y:{})", self.x, self.y)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FallingBytes(pub Vec<BytePosition>);

impl Display for FallingBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FallingBytes(falling_bytes) = self;
        write!(f, "[{}]", vector_display(falling_bytes, ","))
    }
}