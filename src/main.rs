mod reading;
mod answer;
mod parser;
mod solver;
mod verifier;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod executer;
mod reader;
mod pipelined_executer;
mod executer_manager;
mod arguments;
mod helper;
mod testing;

use answer::Answer;
use arguments::Arguments;
use executer_manager::ExecuterManager;
use reading::*;


fn create_executer_manager() -> Result<ExecuterManager, String> {
    ExecuterManager::new()
        .and_then(day_01::register)
        .and_then(day_02::register)
        .and_then(day_03::register)
        .and_then(day_04::register)
        .and_then(day_05::register)
        .and_then(day_06::register)
        .and_then(day_07::register)
        .and_then(day_08::register)
        .and_then(day_09::register)
        .and_then(day_10::register)
        .and_then(day_11::register)
        .and_then(day_12::register)
        .and_then(day_13::register)
        .and_then(day_14::register)
        .and_then(day_15::register)
        .and_then(day_16::register)
        .and_then(day_17::register)
        .and_then(day_18::register)
        .and_then(day_19::register)
        .and_then(day_20::register)
}

fn report_outcome(outcome: Result<Answer, String>) {
    match outcome {
        Ok(report) => print!("{}", report.report()),
        Err(error) => {
            println!("An error occurred during processing.");
            print!("{}", error)
        },
    } 
}

fn get_outcome(arg: Arguments) -> Result<Answer, String> {
    let day = arg.optional_day;
    let is_part_2 = arg.is_part_2;
    arg.optional_filepath
        .ok_or(format!("Input filepath is not provided"))
        .and_then(|filepath|create_executer_manager()?.try_execute_executer(&filepath, day, is_part_2))
}

fn main()
{
    let args = std::env::args().collect::<Vec<_>>();
    let arguments = arguments::Arguments::new(&args);
    report_outcome(get_outcome(arguments));
}