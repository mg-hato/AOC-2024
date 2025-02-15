use lowest_score_calculator::LowestScoreCalculator;
use model::Field;
use parser::ReindeerMazeParser;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::{SanitisedFileReader, SimpleFileReader}, solver::Solve, verifier::TrivialVerifier};

mod model;
mod parser;
mod test;
mod lowest_score_calculator;

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

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Table<Field>>, String>
where S: Solve<Table<Field>> + 'static {
    try_make_pipeline(
        Ok(reader()),
        Ok(ReindeerMazeParser::new()),
        Ok(TrivialVerifier::new::<Table<Field>>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<Field>>, String> {
    make_pipeline_with(LowestScoreCalculator)
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(16, false, make_pipeline(false), false)
        ?.try_register(16, true, make_pipeline(true), false)
}