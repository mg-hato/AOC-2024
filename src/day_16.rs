use reindeer_maze_solver::ReindeerMazeSolver;
use model::Field;
use parser::ReindeerMazeParser;
use reindeer_path_analyser::{LowestScoreAnalyser, OptimalPathFieldAnalyser};

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::{SanitisedFileReader, SimpleFileReader}, solver::Solve, verifier::TrivialVerifier};

mod model;
mod parser;
mod test;
mod state;
mod reindeer_path_analyser;
mod reindeer_maze_solver;

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
    match is_part_2 {
        false => make_pipeline_with(ReindeerMazeSolver::new(LowestScoreAnalyser)),
        true  => make_pipeline_with(ReindeerMazeSolver::new(OptimalPathFieldAnalyser)),
    }
    
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(16, false, make_pipeline(false), false)
        ?.try_register(16, true, make_pipeline(true), false)
}