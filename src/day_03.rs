use ExecuterManager;

use crate::{parser::TrivialParser, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reader::VecLine, solver::Solve, verifier::TrivialVerifier, SimpleFileReader};

mod mul_extractor;
mod instruction;
mod test;

use self::mul_extractor::MulExtractor;

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<VecLine>, String>
where S: Solve<VecLine> + 'static {
    try_make_pipeline(
        Ok(SimpleFileReader::new()),
        Ok(TrivialParser::new()),
        Ok(TrivialVerifier::new::<VecLine>()),
        solver)
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<VecLine>, String> {
    make_pipeline_with(MulExtractor::new(is_part_2))
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(3, false, make_pipeline(false), false)
        ?.try_register(3, true, make_pipeline(true), false)
}

