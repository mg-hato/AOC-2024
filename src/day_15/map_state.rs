use std::collections::HashMap;

use crate::helper::position::UPosition;

pub struct MapState {
    /// Crates existing on the map, a crate with ID `i` is on position `i` (`crates[i]`)
    pub crates: Vec<UPosition>,

    /// Map is a hash map from position to id of crate occupying it (if any).
    /// If a position `P` is in the map, it means it is non-wall position. The associated value can be:
    /// - `Some(id)` representing the `id` of the crate occupying it.
    /// - `None` if the position is free of crates.
    pub map: HashMap<UPosition, Option<usize>>,

    /// Position of the robot
    pub robot: UPosition,
}