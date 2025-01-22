
use crate::{answer::{Answer, DisplayableAnswer}, helper::{boundary::Boundary, movement, position::UPosition, table::Table}, solver::Solve};

use super::review::Review;

pub struct TrailheadReviewAnalyser<R> where R: Review {
    review_provider: Box<dyn Fn() -> R>
}

impl <R: Review> TrailheadReviewAnalyser<R> {
    pub fn new<RF>(review_provider: RF) -> TrailheadReviewAnalyser<R> where RF: 'static + Fn() -> R {
        TrailheadReviewAnalyser { review_provider: Box::new(review_provider) }
    }

    /// Reviews given position as a candidate for trailhead. If it is a trailhead,
    /// it will calculate its review value using reviewer created by the `reviewer_provider`.
    fn review(&self, map: &Table<usize>, start_position: UPosition) -> usize {
        // Review only when start position is of height 0
        if map.get_pos(start_position).is_none_or(|&height|height != 0) { return 0; }
        
        let mut reviewer = (self.review_provider)();

        let mut positions = vec![start_position];
        while !positions.is_empty() {
            let current = positions.pop().unwrap();
            let height = *map.get_pos(current).unwrap();

            // End of path with height 9, register it and skip processing further    
            if height == 9 {
                reviewer.register(current);
                continue;
            }

            // Analyse all 4 directions and see if the  it is an even, gradual, uphill slope
            for movement in movement::unit::all_partial() {
                let next = map.boundary().apply(movement, current);
                if next.is_some_and(|p|*map.get_pos(p).unwrap() == height + 1) {
                    positions.push(next.unwrap());
                }
            }
        }
        reviewer.review()
    }
}

impl <R> Solve<Table<usize>> for TrailheadReviewAnalyser<R> where R: Review {
    fn solve(&self, input: Table<usize>) -> Result<Answer, String> {
        Ok(DisplayableAnswer::new(input.iter().map(|(pos, _)|self.review(&input, pos)).sum::<usize>()))
    }
}