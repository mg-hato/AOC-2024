use cheats_counter::CheatsCounter;
use model::Field;
use parser::RacetrackParser;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::{SanitisedFileReader, SimpleFileReader}, solver::Solve, verifier::TrivialVerifier};

mod parser;
mod model;
mod test;
mod cheats_counter;

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
        Ok(RacetrackParser::new()),
        Ok(TrivialVerifier::new::<Table<Field>>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<Field>>, String> {
    match is_part_2 {
        false => make_pipeline_with(CheatsCounter::new(2, 100)),
        true => make_pipeline_with(CheatsCounter::new(20, 100)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(20, false, make_pipeline(false), false)
        ?.try_register(20, true, make_pipeline(true), false)
}