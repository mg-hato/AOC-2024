use crate::day_02::level_report_analyser::LevelReportAnalyser;

use super::{level_report_counter::LevelReportCounter, models::LevelReport};

pub struct DampenedSafeLevelReportCounter {
    safety_fn: Box<dyn Fn(u32, u32) -> bool>
}

impl DampenedSafeLevelReportCounter {
    pub fn new<SF>(safety_fn: SF) -> DampenedSafeLevelReportCounter
    where SF : Fn(u32, u32) -> bool + 'static {
        DampenedSafeLevelReportCounter { safety_fn: Box::new(safety_fn) }
    }

    /// Examines whether the report would be safe if the level at index `i` were to be removed.
    /// This process involes establishing a disjoint prefix and suffix sub-sequence where neither contains
    /// the i-th level. Further steps involve establishing whether joining these two would make for a safe sequence.
    /// Considerations:
    /// - Both are already safe sequences on their own,
    /// - The leading levels of both are forming a "safe step" (safety function satisfied)
    /// - Both sequences have the same increasing or decreasing trend
    /// - The mini-sequence formed of the leading levels of the two sequences establishes the same increasing or decreasing trend
    fn is_safe_without_level(&self, analyser: &LevelReportAnalyser, i: usize) -> bool {
        // Prefix (left): ignore the i-th level, take prefix `report[0..i)`
        let left_status = analyser.prefix(i);
        let (left_level, left_trend) = (left_status.level(), left_status.trend_boolean());
        
        // Suffix (right): ignore the i-th level, take suffix `report[i+1..n)` (n is the length)
        let right_status = analyser.suffix(i + 1);
        let (right_level, right_trend) = (right_status.level(), right_status.trend_boolean());

        // Are both prefix and suffix safe on their own. If not, return early
        if !left_status.is_safe() || !right_status.is_safe() {
            return false;
        }

        // Are levels on both prefix and suffix satisfying safety function, if not, return early.
        // If they are safe, "calculate" the established trend formed by the two levels
        let estabslished_trend = if let (Some(left), Some(right)) = (left_level, right_level) {
            if !(self.safety_fn)(left, right) { return false; }
            Some(left < right)
        } else { None };

        // Are the trends on prefix and on suffix matching, if not, return early.
        if let (Some(left), Some(right)) = (left_trend, right_trend) {
            if left != right { return false; }
        }
        
        // Lastly, is the established trend in line with already existing trend (from prefix of from suffix)
        match (estabslished_trend, left_trend, right_trend) {
            (Some(established_trend), Some(left), _) => established_trend == left,
            (Some(established_trend), _, Some(right)) => established_trend == right,
            _ => true
        }
    }
}

impl LevelReportCounter for DampenedSafeLevelReportCounter {
    fn predicate(&self, report: &LevelReport) -> bool {
        let analyser = LevelReportAnalyser::new(report, &self.safety_fn);
        if analyser.suffix(0).is_safe() {
            return true;
        }

        for i in 0..report.levels.len() {
            if self.is_safe_without_level(&analyser, i) {
                return true;
            } 
        }
        false
    }
}