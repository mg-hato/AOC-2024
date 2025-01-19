use crate::helper::position::UPosition;

/// Trailhead review abstraction
pub trait Review {
    /// Registers position for the purposes of creating review value
    fn register(&mut self, position: UPosition);

    /// Provides review value based on position data provided
    fn review(&self) -> usize;
}