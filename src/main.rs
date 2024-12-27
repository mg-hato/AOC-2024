mod reading;
mod answer;
mod parser;
mod solver;
mod verifier;
mod day_01;
mod day_02;
mod day_03;
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