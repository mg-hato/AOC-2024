use box_prediction_model::BoxPredictionModel;
use models::MapAndMoves;
use parser::MapAndMovesParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::{SanitisedFileReader, SimpleFileReader}, solver::Solve, verifier::TrivialVerifier};

mod models;
mod parser;
mod test;
mod map_state;
mod box_prediction_model;

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

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<MapAndMoves>, String>
where S: Solve<MapAndMoves> + 'static {
    try_make_pipeline(
        Ok(reader()),
        Ok(MapAndMovesParser),
        Ok(TrivialVerifier::new::<MapAndMoves>()),
        solver
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<MapAndMoves>, String> {
    let scale = match is_part_2 {
        false => 1,
        true  => 2,
    };
    make_pipeline_with(BoxPredictionModel::new(scale))
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(15, false, make_pipeline(false), false)
        ?.try_register(15, true, make_pipeline(true), false)
}