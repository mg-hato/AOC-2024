use models::RobotList;
use parser::RobotListParser;
use safety_factor_calculator::SafetyFactorCalculator;
use snapshot_capturer::SnapshotCapturer;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod parser;
mod models;
mod test;
mod safety_factor_calculator;
mod robots_prediction_model;
mod snapshot_capturer;
mod snapshots_answer;

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<RobotList>, String>
where S: Solve<RobotList> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        RobotListParser::new(),
        Ok(TrivialVerifier::new::<RobotList>()),
        solver)
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<RobotList>, String> {
    match is_part_2 {
        false => make_pipeline_with(SafetyFactorCalculator::new(100, 101, 103)),
        true  => make_pipeline_with(SnapshotCapturer::new(101*103, 101, 103)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(14, false, make_pipeline(false), false)
        ?.try_register(14, true, make_pipeline(true), false)
}