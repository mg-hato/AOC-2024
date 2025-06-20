
use crate::{day_25::{keylock_match_analyser::KeyLockMatchAnalyser, model::KeyLockSchematics, schematics_parser::SchematicsParser}, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::{SanitisedFileReader, SimpleFileReader}, solver::Solve, verifier::TrivialVerifier};

mod model;
mod schematics_parser;
mod keylock_match_analyser;
mod keylock_converter;
mod key_lock;
mod test;

fn reader() -> SanitisedFileReader {
    use crate::reading::settings::*;
    SanitisedFileReader::new(
        SimpleFileReader,
        LineComment::Pattern(format!("//")),
        InputEndComment::Pattern(format!("===")),
        LineTrim::End,
        EmptyLineTrimming::Both
    )
}

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<KeyLockSchematics>, String>
where S: Solve<KeyLockSchematics> + 'static {
    try_make_pipeline(
        Ok(reader()),
        Ok(SchematicsParser::new()),
        Ok(TrivialVerifier::new::<KeyLockSchematics>()),
        Ok(solver)
    )
}

fn make_pipeline() -> Result<PipelinedExecuter<KeyLockSchematics>, String> {
    make_pipeline_with(KeyLockMatchAnalyser)
}


pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(25, false, make_pipeline(), false)
}