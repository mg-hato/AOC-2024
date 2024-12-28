use self::cross_mas_finder::CrossMasFinder;
use self::xmas_finder::XMasFinder;
use self::word_search_parser::WordSearchParser;
use self::word_search_verifier::WordSearchVerifier;
use self::word_searcher::WordSearcher;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reader::VecLine, solver::Solve, SanitisedFileReader};

mod word_search_parser;
mod word_searcher;
mod word_search_verifier;
mod xmas_finder;
mod cross_mas_finder;
mod find;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<VecLine>, String>
where S: Solve<VecLine> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        WordSearchParser::new(), 
        Ok(WordSearchVerifier::new()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<VecLine>, String> {
    match is_part_2 {
        false => make_pipeline_with(WordSearcher::new(XMasFinder::new)),
        true => make_pipeline_with(WordSearcher::new(CrossMasFinder::new)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(4, false, make_pipeline(false), false)
        ?.try_register(4, true, make_pipeline(true), false)
}