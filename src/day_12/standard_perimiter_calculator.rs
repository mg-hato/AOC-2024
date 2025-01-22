use std::collections::HashSet;

use super::{fence_unit::FenceUnit, perimiter_calculate::PerimiterCalculate};

/// Calculates the perimeter multiplier as the number of fence units required.
/// According the modern day business practices.
pub struct StandardPerimiterCalculator;

impl PerimiterCalculate for StandardPerimiterCalculator {
    fn calculate(&self, fence: HashSet<FenceUnit>) -> u64 {
        fence.len() as u64
    }
}