use self::models::LaboratoryMap;
use self::parser::LaboratoryMapParser;
use self::verifier::LaboratoryMapVerifier;
use self::distinct_visiting_positions_counter::DistinctVisitingPositionsCounter;
use self::loop_candidate_counter::LoopCandidateCounter;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, settings::{EmptyLineTrimming, InputEndComment, LineComment, LineTrim}, solver::Solve, SanitisedFileReader, SimpleFileReader};

mod parser;
mod models;
mod verifier;
mod distinct_visiting_positions_counter;
mod loop_candidate_counter;
mod test;
mod map_analyser;
mod direction;
mod guard_state;
mod loop_detector;
mod next_state;
mod optimised_caching_loop_detector;
mod adjusted_loop_detector;

fn make_reader() -> SanitisedFileReader {
    SanitisedFileReader::new(
        SimpleFileReader::new(),
        LineComment::Pattern(format!("//")),
        InputEndComment::Pattern(format!("===")), // we had to change this, as "###" might be part of input
        LineTrim::Both,
        EmptyLineTrimming::All,
    )
}

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<LaboratoryMap>, String>
where S: Solve<LaboratoryMap> + 'static {
    try_make_pipeline(Ok(make_reader()), LaboratoryMapParser::new(), Ok(LaboratoryMapVerifier::new()), Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<LaboratoryMap>, String> {
    match is_part_2 {
        false => make_pipeline_with(DistinctVisitingPositionsCounter),
        true  => make_pipeline_with(LoopCandidateCounter)
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager
        .try_register(6, false, make_pipeline(false), false)
        ?.try_register(6, true, make_pipeline(true), false)
}