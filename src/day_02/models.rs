use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LevelReport {
    pub levels: Vec<u32>
}

impl LevelReport {
    pub fn new(levels: Vec<u32>) -> LevelReport {
        LevelReport { levels }
    }
}

impl Display for LevelReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Levels[{}]", vector_display(&self.levels, ","))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LevelReports {
    pub reports: Vec<LevelReport>
}

impl LevelReports {
    pub fn new(reports: Vec<LevelReport>) -> LevelReports {
        LevelReports { reports }
    }
}

impl Display for LevelReports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level Reports[{}]", vector_display(&self.reports, ","))
    }
}