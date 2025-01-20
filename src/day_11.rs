use self::stone_prediction_model::StonePredictionModel;
use self::model::Stones;
use self::parser::StonesParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader};

mod model;
mod parser;
mod stone_prediction_model;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Stones>, String>
where S: Solve<Stones> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        StonesParser::new(),
        Ok(TrivialVerifier::new::<Stones>()),
        Ok(solver),
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Stones>, String> {
    match is_part_2 {
        false => make_pipeline_with(StonePredictionModel::new(25)),
        true  => make_pipeline_with(StonePredictionModel::new(75)),
    }
}



pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(11, false, make_pipeline(false), false)
        ?.try_register(11, true, make_pipeline(true), false)
}