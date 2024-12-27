use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::models::{LevelReport, LevelReports};

pub trait LevelReportCounter {
    fn predicate(&self, report: &LevelReport) -> bool;
}

impl<LRC: LevelReportCounter> Solve<LevelReports> for LRC {
    fn solve(&self, input: LevelReports) -> Result<Answer, String> {
        let count = input.reports.into_iter().filter(|report|self.predicate(report)).count();
        Ok(DisplayableAnswer::new(count))
    }
}