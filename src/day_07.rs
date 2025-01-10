use self::operation::{Operation, Addition, Multiplication, Concatenation};
use self::calibration_results_checker::CalibrationResultsChecker;
use self::equation::EquationList;
use self::parser::EquationListParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader};

mod equation;
mod parser;
mod operation;
mod test;
mod calibration_results_checker;


fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<EquationList>, String>
where S: Solve<EquationList> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        EquationListParser::new(),
        Ok(TrivialVerifier::new::<EquationList>()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<EquationList>, String> {
    let mut ops : Vec<Box<dyn Operation>> = vec![Box::new(Addition), Box::new(Multiplication)];
    if is_part_2 { ops.push(Box::new(Concatenation)); }
    make_pipeline_with(CalibrationResultsChecker::new(ops))
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager
        .try_register(7, false, make_pipeline(false), false)
        ?.try_register(7, true, make_pipeline(true), false)
}