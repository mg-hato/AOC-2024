use self::discounted_perimiter_calculator::DiscountedPerimiterCalculator;
use self::standard_perimiter_calculator::StandardPerimiterCalculator;
use self::fence_price_calculator::FencePriceCalculator;
use self::parser::GardenParser;

use crate::{executer_manager::ExecuterManager, helper::table::Table, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, solver::Solve, verifier::TrivialVerifier, SanitisedFileReader, SimpleFileReader};


mod parser;
mod fence_price_calculator;
mod fence_unit;
mod perimiter_calculate;
mod standard_perimiter_calculator;
mod discounted_perimiter_calculator;
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
        Ok(GardenParser), 
        Ok(TrivialVerifier::new::<Table<char>>()),
        Ok(solver))
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Table<char>>, String> {
    match is_part_2 {
        false => make_pipeline_with(FencePriceCalculator::new(StandardPerimiterCalculator)),
        true  => make_pipeline_with(FencePriceCalculator::new(DiscountedPerimiterCalculator)),
    }
}

pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(12, false, make_pipeline(false), false)
        ?.try_register(12, true, make_pipeline(true), false)
}