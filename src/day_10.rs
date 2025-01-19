use self::parser::TopographicMapParser;
use self::trailhead_counter::TrailheadCounter;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader, SimpleFileReader};

mod parser;
mod trailhead_counter;
mod test;

fn reader() -> SanitisedFileReader {
    SanitisedFileReader::new(
        SimpleFileReader,
        crate::settings::LineComment::Pattern(format!("//")),
        crate::settings::InputEndComment::Pattern(format!("####")),
        crate::settings::LineTrim::End,
        crate::settings::EmptyLineTrimming::Both)
}

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Table<usize>>, String>
where S: Solve<Table<usize>> + 'static {
    try_make_pipeline(
        Ok(reader()),
        Ok(TopographicMapParser),
        Ok(TrivialVerifier::new::<Table<usize>>()),
        Ok(solver),
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<usize>>, String> {
    match is_part_2 {
        false => make_pipeline_with(TrailheadCounter),
        true  => make_pipeline_with(TrailheadCounter),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(10, false, make_pipeline(false), false)
        // ?.try_register(10, true, make_pipeline(true), false)
}