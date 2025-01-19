use self::cross_mas_finder::CrossMasFinder;
use self::xmas_finder::XMasFinder;
use self::word_search_parser::WordSearchParser;
use self::word_searcher::WordSearcher;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader, SimpleFileReader};

mod word_search_parser;
mod word_searcher;
mod xmas_finder;
mod cross_mas_finder;
mod find;
mod test;

fn reader() -> SanitisedFileReader {
    use crate::settings::*;

    SanitisedFileReader::new(
        SimpleFileReader,
        LineComment::Pattern(format!("//")),
        InputEndComment::Pattern(format!("###")),
        LineTrim::End,
        EmptyLineTrimming::Both)
}

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Table<char>>, String>
where S: Solve<Table<char>> + 'static {
    try_make_pipeline(
        Ok(reader()),
        WordSearchParser::new(), 
        Ok(TrivialVerifier::new::<Table<char>>()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<char>>, String> {
    match is_part_2 {
        false => make_pipeline_with(WordSearcher::new(XMasFinder::new)),
        true => make_pipeline_with(WordSearcher::new(CrossMasFinder::new)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(4, false, make_pipeline(false), false)
        ?.try_register(4, true, make_pipeline(true), false)
}