use model::TowelPatternsAndDesigns;
use towel_design_checker::{DifferentWaysInterpreter, PossibilityInterpreter, TowelDesignChecker};
use towel_patterns_and_designs_parser::TowelPatternsAndDesignsParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod towel_patterns_and_designs_parser;
mod towel_design_checker;
mod test;


fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<TowelPatternsAndDesigns>, String>
where S: Solve<TowelPatternsAndDesigns> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        TowelPatternsAndDesignsParser::new(),
        Ok(TrivialVerifier::new::<TowelPatternsAndDesigns>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<TowelPatternsAndDesigns>, String> {
    match is_part_2 {
        false => make_pipeline_with(TowelDesignChecker::new(PossibilityInterpreter)),
        true  => make_pipeline_with(TowelDesignChecker::new(DifferentWaysInterpreter)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(19, false, make_pipeline(false), false)
        ?.try_register(19, true, make_pipeline(true), false)
}