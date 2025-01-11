use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, helper::table::Table, solver::Solve};

use super::{map_analyser::MapAnalyser, models::LaboratoryMapField};

pub struct DistinctVisitingPositionsCounter;

impl Solve<Table<LaboratoryMapField>> for DistinctVisitingPositionsCounter {
    fn solve(&self, input: Table<LaboratoryMapField>) -> Result<Answer, String> {
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