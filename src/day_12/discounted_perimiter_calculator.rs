use std::collections::HashSet;

use super::{fence_unit::FenceUnit, perimiter_calculate::PerimiterCalculate};

/// Calculates perimiter price multiplier coefficient as the number of sides the region has.
/// Multiple fence units are considered a single side if they are forming a straight continuous line.
pub struct DiscountedPerimiterCalculator;

impl PerimiterCalculate for DiscountedPerimiterCalculator {
    fn calculate(&self, fence: HashSet<FenceUnit>) -> u64 {
        let mut done = HashSet::new();
        let mut multiplier = 0;
        for &fence_unit in fence.iter() {
            if done.contains(&fence_unit) { continue; }
            done.insert(fence_unit);
            let FenceUnit { position, dir } = fence_unit;

            for movement in [dir.rotate(), dir.rotate().rotate().rotate()].iter().map(|d|d.movement()) {
                let mut current_position = Some(position);
                while let Some(pos) = current_position {
                    let pos_fence_unit = FenceUnit::new(pos, dir);
                    if !fence.contains(&pos_fence_unit) { break; }
                    done.insert(FenceUnit::new(pos, dir));
                    current_position = movement.apply(pos);
                }
            }
            multiplier += 1;
        }
        multiplier
    }
}