use crate::{day_22::{model::Numbers, numbers_parser::NumbersParser, optimal_change_sequence_finder::OptimalChangeSequenceFinder, secret_number_examiner::SecretNumberExaminer}, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod numbers_parser;
mod secret_number_transform;
mod secret_number_examiner;
mod optimal_change_sequence_finder;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Numbers>, String>
where S: Solve<Numbers> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        NumbersParser::new(),
        Ok(TrivialVerifier::new::<Numbers>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Numbers>, String> {
    match is_part_2 {
        false => make_pipeline_with(SecretNumberExaminer::new(2_000)),
        true => make_pipeline_with(OptimalChangeSequenceFinder::new(2_000, 4)),
    }
}


pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(22, false, make_pipeline(false), false)
        ?.try_register(22, true, make_pipeline(true), false)
}