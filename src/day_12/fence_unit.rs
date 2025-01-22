use crate::helper::{direction::Direction, position::UPosition};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
/// Represents the unit of fence where `position` is the `UPosition`
/// belonging to the region and `dir` is an immediate `Direction` of where the fence is. 
pub struct FenceUnit {
    pub position: UPosition,
    pub dir: Direction,
}

impl FenceUnit {
    pub fn new(position: UPosition, dir: Direction) -> FenceUnit {
        FenceUnit { position, dir }
    }
}