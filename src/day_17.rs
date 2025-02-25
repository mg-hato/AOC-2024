use model::ProgramInformation;
use parser::ProgramInformationParser;
use program_copy_resolver::ProgramCopyResolver;
use program_simulator::ProgramSimulator;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod parser;
mod program_simulator;
mod program_copy_resolver;
mod test;


fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<ProgramInformation>, String>
where S: Solve<ProgramInformation> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        ProgramInformationParser::new(),
        Ok(TrivialVerifier::new::<ProgramInformation>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<ProgramInformation>, String> {
    match is_part_2 {
        false => make_pipeline_with(ProgramSimulator::new(200)),
        true  => make_pipeline_with(ProgramCopyResolver::new(200)),
    }
    
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(17, false, make_pipeline(false), false)
        ?.try_register(17, true, make_pipeline(true), false)
}