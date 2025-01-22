use std::collections::HashSet;

use super::fence_unit::FenceUnit;

/// Calculates the perimiter price multiplier coefficient 
pub trait PerimiterCalculate {
    fn calculate(&self, fence: HashSet<FenceUnit>) -> u64;
}