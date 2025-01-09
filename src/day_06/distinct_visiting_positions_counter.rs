use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::{map_analyser::MapAnalyser, models::LaboratoryMap};

pub struct DistinctVisitingPositionsCounter;

impl Solve<LaboratoryMap> for DistinctVisitingPositionsCounter {
    fn solve(&self, input: LaboratoryMap) -> Result<Answer, String> {
        MapAnalyser::new(input)
            .and_then(|mut analyser|analyser.perform_analysis())
            .and_then(|path|match path.is_empty() {
                true => Err(format!("Loops")),
                false => {
                    let mut distinct_positions = HashSet::new();
                    for state in path { distinct_positions.insert(state.position); }
                    Ok(distinct_positions.len())
                },
            }).map(DisplayableAnswer::new)
    }
}