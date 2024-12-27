use super::{level_sequence_status::LevelSequenceStatus::{self, *}, models::LevelReport};

pub struct LevelReportAnalyser {
    prefixed: Vec<LevelSequenceStatus>,
    suffixed: Vec<LevelSequenceStatus>,
}

impl LevelReportAnalyser {
    pub fn new<SF>(report: &LevelReport, safety_fn: &SF) -> LevelReportAnalyser
    where SF: Fn(u32, u32) -> bool + 'static{
        LevelReportAnalyser {
            prefixed: Self::forward_analysis(safety_fn, report),
            suffixed: Self::backward_analysis(safety_fn, report),
        }
    }

    /// Returns level sequence state for subsequence of underlying report
    /// defined by report[0..i).
    pub fn prefix(&self, i: usize) -> &LevelSequenceStatus {
        let index = i.min(self.prefixed.len() - 1);
        &self.prefixed[index]
    }

    /// Returns level sequence status for subsequence of underlying report
    /// defined by report[i..n) where `n` is the number of levels in the report.
    /// The sequence is growing on the front end, meaning the smaller the value of
    /// `i`, the longer the sequence becomes
    pub fn suffix(&self, i: usize) -> &LevelSequenceStatus {
        let index = i.min(self.suffixed.len() - 1);
        &self.suffixed[index]
    }

    fn forward_analysis<SF>(safety_fn: &SF, report: &LevelReport) -> Vec<LevelSequenceStatus>
    where SF: Fn(u32, u32) -> bool + 'static {
        let mut analysis = vec![LevelSequenceStatus::NoData];
        for i in 0..report.levels.len() {
            analysis.push(Self::next_status(safety_fn, &analysis[i], report.levels[i]));
        }
        analysis
    }

    fn reverse(status: LevelSequenceStatus) -> LevelSequenceStatus {
        match status {
            LevelSequenceStatus::Trend(level, is_increasing) => LevelSequenceStatus::Trend(level, !is_increasing),
            other => other,
        }
    }

    fn backward_analysis<SF>(safety_fn: &SF, report: &LevelReport) -> Vec<LevelSequenceStatus>
    where SF: Fn(u32, u32) -> bool + 'static {
        let mut cloned_report = report.clone();
        cloned_report.levels.reverse();
        Self::forward_analysis(safety_fn, &cloned_report).into_iter().map(Self::reverse).rev().collect()
    }

    fn next_status<SF>(safety_fn: &SF, status: &LevelSequenceStatus, next: u32) -> LevelSequenceStatus
    where SF: Fn(u32, u32) -> bool + 'static {
        match &status {
            NoData => Single(next),
            Single(current)
                if safety_fn(*current, next) && *current != next
                => Trend(next, *current < next),
            Trend(current,is_increasing)
                if safety_fn(*current, next) && *current != next
                && (*current < next) == *is_increasing
                => Trend(next, *is_increasing),
            _ => Unsafe,
        }
    } 
}