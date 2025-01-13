
use self::file_by_file_compacter::FileByFileCompacter;
use self::block_by_block_compacter::BlockByBlockCompacter;
use self::disk_compacter::DiskCompacter;
use self::model::DiskMap;
use self::parser::DiskMapParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader};

mod parser;
mod model;
mod disk_compacter;
mod compact;
mod block_by_block_compacter;
mod file_by_file_compacter;
mod test;
mod memory_block;
mod pos_size;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<DiskMap>, String>
where S: Solve<DiskMap> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        Ok(DiskMapParser),
        Ok(TrivialVerifier::new::<DiskMap>()),
        Ok(solver),
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<DiskMap>, String> {
    match is_part_2 {
        false => make_pipeline_with(DiskCompacter::new(BlockByBlockCompacter)),
        true  => make_pipeline_with(DiskCompacter::new(FileByFileCompacter)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(9, false, make_pipeline(false), false)
        ?.try_register(9, true, make_pipeline(true), false)
}