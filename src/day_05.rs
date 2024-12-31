use self::updates_corrector::UpdatesCorrector;
use self::updates_checker::UpdatesChecker;
use self::rules_with_updates_parser::RulesWithUpdatesParser;
use self::verifier::RulesWithUpdatesVerifier;
use self::models::RulesWithUpdates;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, SanitisedFileReader};

mod models;
mod rules_with_updates_parser;
mod verifier;
mod updates_checker;
mod updates_corrector;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<RulesWithUpdates>, String>
where S: Solve<RulesWithUpdates> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        RulesWithUpdatesParser::new(),
        Ok(RulesWithUpdatesVerifier::new()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<RulesWithUpdates>, String> {
    match is_part_2 {
        false => make_pipeline_with(UpdatesChecker::new()),
        true => make_pipeline_with(UpdatesCorrector::new()),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(5, false, make_pipeline(false), false)
        ?.try_register(5, true, make_pipeline(true), false)
}