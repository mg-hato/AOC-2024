use self::dampened_safe_lelel_report_counter::DampenedSafeLevelReportCounter;
use self::safe_level_report_counter::SafeLevelReportCounter;
use self::models::LevelReports;
use self::parser::LevelReportsParser;

use crate::{executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader};

mod models;
mod parser;
mod test;
mod safe_level_report_counter;
mod dampened_safe_lelel_report_counter;
mod level_report_analyser;
mod level_sequence_status;
mod level_report_counter;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<LevelReports>, String>
where S: Solve<LevelReports> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        LevelReportsParser::new(),
        Ok(TrivialVerifier::new::<LevelReports>()),
        Ok(solver)
    )
}


fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<models::LevelReports>, String> {
    let safety_fn = |current:u32, next:u32|{
        let absdiff = current.abs_diff(next);
        1 <= absdiff && absdiff <= 3
    };
    match is_part_2 {
        false => make_pipeline_with(SafeLevelReportCounter::new(safety_fn)),
        true => make_pipeline_with(DampenedSafeLevelReportCounter::new(safety_fn)),
    }
    
}


pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager
        .try_register(2, false, make_pipeline(false), false)
        ?.try_register(2, true, make_pipeline(true), false)
}