use crate::day_02::level_report_analyser::LevelReportAnalyser;

use super::{level_report_counter::LevelReportCounter, models::LevelReport};


pub struct SafeLevelReportCounter {
    safety_fn: Box<dyn Fn(u32, u32) -> bool>
}

impl SafeLevelReportCounter {
    pub fn new<SF>(safety_fn: SF) -> SafeLevelReportCounter
    where SF : Fn(u32, u32) -> bool + 'static {
        SafeLevelReportCounter { safety_fn: Box::new(safety_fn) }
    }
}

impl LevelReportCounter for SafeLevelReportCounter {
    fn predicate(&self, report: &LevelReport) -> bool {
        let analyser = LevelReportAnalyser::new(report, &self.safety_fn);
        analyser.suffix(0).is_safe()
    }
}