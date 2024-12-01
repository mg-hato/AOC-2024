mod parser;
mod distance_apart_calculator;
mod similarity_score_calculator;
mod models;
mod test;

use self::similarity_score_calculator::SimilarityScoreCalculator;
use self::distance_apart_calculator::DistanceApartCalculator;

use crate::executer_manager::ExecuterManager;
use crate::pipelined_executer::try_make_pipeline;
use crate::pipelined_executer::PipelinedExecuter;
use crate::solver::Solve;
use crate::verifier::TrivialVerifier;
use crate::SanitisedFileReader;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<models::NumberPairList>, String>
where S: Solve<models::NumberPairList> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        parser::NumberPairListParser::new(),
        Ok(TrivialVerifier::new::<models::NumberPairList>()),
        Ok(solver),
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<models::NumberPairList>, String> {
    match is_part_2 {
        false => make_pipeline_with(DistanceApartCalculator::new()),
        true => make_pipeline_with(SimilarityScoreCalculator::new()),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager
        .try_register(1, false, make_pipeline(false), false)
        ?.try_register(1, true, make_pipeline(true), false)
}