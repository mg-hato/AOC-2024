use claw_machines_analyser::ClawMachineAnalyser;
use model::ClawMachines;
use parser::ClawMachinesParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader};


mod model;
mod parser;
mod claw_machines_analyser;
mod single_solution_solver;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<ClawMachines>, String>
where S: Solve<ClawMachines> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        ClawMachinesParser::new(), 
        Ok(TrivialVerifier::new::<ClawMachines>()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<ClawMachines>, String> {
    let tweak = match is_part_2 {
        false => 0,
        true  => 10_000_000_000_000,
    };
    make_pipeline_with(ClawMachineAnalyser::new_with_tweak(tweak, tweak))
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(13, false, make_pipeline(false), false)
        ?.try_register(13, true, make_pipeline(true), false)
}