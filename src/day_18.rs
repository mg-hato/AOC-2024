use falling_bytes_parser::FallingBytesParser;
use first_byte_blocker_finder::FirstByteBlockerFinder;
use memory_space_path_finder::MemorySpacePathFinder;
use model::FallingBytes;

use crate::{executer_manager::ExecuterManager, helper::position::UPosition, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod falling_bytes_parser;
mod memory_space_path_finder;
mod first_byte_blocker_finder;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<FallingBytes>, String>
where S: Solve<FallingBytes> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        FallingBytesParser::new(),
        Ok(TrivialVerifier::new::<FallingBytes>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<FallingBytes>, String> {
    let bottom_right_corner = UPosition::new((70, 70));
    
    match is_part_2 {
        false => make_pipeline_with(MemorySpacePathFinder::new(bottom_right_corner, 1024)),
        true  => make_pipeline_with(FirstByteBlockerFinder::new(bottom_right_corner)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(18, false, make_pipeline(false), false)
        ?.try_register(18, true, make_pipeline(true), false)
}