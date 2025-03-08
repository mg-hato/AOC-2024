use code_parser::CodeParser;
use model::Codes;

use crate::{day_21::keypad_complexity_calculator::KeypadComplexityCalculator, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod code_parser;
mod keypad;
mod ordered_movement;
mod caching;
mod keypad_complexity_calculator;
mod test;

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<Codes>, String>
where S: Solve<Codes> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        CodeParser::new(),
        Ok(TrivialVerifier::new::<Codes>()),
        solver
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Codes>, String> {
    match is_part_2 {
        false => make_pipeline_with(KeypadComplexityCalculator::new(6, 2)),
        true => make_pipeline_with(KeypadComplexityCalculator::new(6, 25)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(21, false, make_pipeline(false), false)
        ?.try_register(21, true, make_pipeline(true), false)
}