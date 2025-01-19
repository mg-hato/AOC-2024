use crate::helper::position::UPosition;

use super::review::Review;


/// A trailhead rating is a number of different paths ending at position of height 9. 
/// It is a glorified counter implementing `Review` interface.
pub struct TrailheadRating {
    counter: usize,
}

impl TrailheadRating {
    pub fn new() -> TrailheadRating { TrailheadRating { counter: 0 } }
}

impl Review for TrailheadRating {
    /// For every distinct path, just invoke the method (position supplied is not important)
    fn register(&mut self, _: UPosition) {
        self.counter += 1;
    }

    fn review(&self) -> usize { self.counter }
}