use self::antinode_counter::AntinodeCounter;
use self::simple_antinode_calculator::SimpleAntinodeCalculator;
use self::resonant_harmonics_antinode_calculator::ResonantHarmonicsAntinodeCalculator;
use self::model::AntennaMapField;
use self::parser::AntennaMapParser;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader, SimpleFileReader};

mod parser;
mod antinode_calculator;
mod simple_antinode_calculator;
mod resonant_harmonics_antinode_calculator;
mod model;
mod antinode_counter;
mod test;

fn reader() -> SanitisedFileReader {
    use crate::settings::*;
    SanitisedFileReader::new(
        SimpleFileReader,
        LineComment::Pattern(format!("//")),
        InputEndComment::Pattern(format!("####")),
        LineTrim::End, 
        EmptyLineTrimming::Both)
}

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<Table<AntennaMapField>>, String>
where S: Solve<Table<AntennaMapField>> + 'static {
    try_make_pipeline(
        Ok(reader()),
        Ok(AntennaMapParser),
        Ok(TrivialVerifier::new::<Table<AntennaMapField>>()),
        Ok(solver),
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<AntennaMapField>>, String> {
    match is_part_2 {
        false => make_pipeline_with(AntinodeCounter::new(SimpleAntinodeCalculator::new)),
        true  => make_pipeline_with(AntinodeCounter::new(ResonantHarmonicsAntinodeCalculator::new)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(8, false, make_pipeline(false), false)
        ?.try_register(8, true, make_pipeline(true), false)
}