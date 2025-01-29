use models::{RobotList, XY};
use parser::RobotListParser;
use safety_factor_calculator::SafetyFactorCalculator;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod parser;
mod models;
mod test;
mod safety_factor_calculator;

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<RobotList>, String>
where S: Solve<RobotList> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        RobotListParser::new(),
        Ok(TrivialVerifier::new::<RobotList>()),
        solver)
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<RobotList>, String> {
    make_pipeline_with(SafetyFactorCalculator::new(100, XY { x: 101, y: 103 }))
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(14, false, make_pipeline(false), false)
}