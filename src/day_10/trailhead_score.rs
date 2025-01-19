use std::collections::HashSet;

use crate::helper::position::UPosition;

use super::review::Review;

/// A trailhead score is a number of different positions reachable that are of height 9. 
pub struct TrailheadScore {
    reachable: HashSet<UPosition>,
}

impl TrailheadScore {
    pub fn new() -> TrailheadScore { TrailheadScore { reachable: HashSet::new() } }
}

impl Review for TrailheadScore {
    /// To calculate trailhead score, one needs to register all positions of height 9 reachable from the trailhead.
    fn register(&mut self, position: UPosition) {
        self.reachable.insert(position);
    }

    /// Score value is just equal to the number of distinct positions registered
    fn review(&self) -> usize { self.reachable.len() }
}