use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, day_06::map_analyser::MapAnalyser, helper::table::Table, solver::Solve};

use super::{adjusted_loop_detector::AdjustedLoopDetector, guard_state::GuardState, loop_detector::{loops, LoopDetector}, models::LaboratoryMapField, optimised_caching_loop_detector::OptimisedCachingLoopDetector};

pub struct LoopCandidateCounter;

impl LoopCandidateCounter {
    fn calculate_loop_count(mut map_analyser: MapAnalyser, paths: Vec<GuardState>) -> Result<usize, String> {
        let mut optimised_loop_detector = OptimisedCachingLoopDetector::new(&mut map_analyser);
        let mut skip_set = HashSet::new(); // a growing set of positions to skip
        let mut loops_created = 0;
        
        // the position in which guard starts is immediately part of skip set
        skip_set.insert(optimised_loop_detector.starting_state().position);

        for position in paths.into_iter().map(|state|state.position) {
            // skip if in skip set
            if skip_set.contains(&position) { continue; }
            
            let mut adjusted_loop_detector = AdjustedLoopDetector::new(&mut optimised_loop_detector, position);
            loops_created += match loops(&mut adjusted_loop_detector) {
                Err(error) => return Err(error),
                Ok(is_looping) => if is_looping { 1 } else { 0 }
            };

            // skip position in future
            skip_set.insert(position);
        }

        Ok(loops_created)
    }
}

impl Solve<Table<LaboratoryMapField>> for LoopCandidateCounter {
    fn solve(&self, input: Table<LaboratoryMapField>) -> Result<Answer, String> {
        MapAnalyser::new(input)
            .and_then(|mut analyser|analyser.perform_analysis()
            .and_then(|path|Self::calculate_loop_count(analyser, path)))
            .map(DisplayableAnswer::new)
    }
}