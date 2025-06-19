use crate::{day_24::{assisted_analyser::AssistedAnalyser, model::Wiring, wiring_parser::WiringParser, z_wire_number_calculator::ZWireNumberCalculator}, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};


mod model;
mod wiring_parser;
mod wiring_simulator;
mod z_wire_number_calculator;
mod wires_extractor;
mod assisted_analyser;
mod wire_simulation_scenario;
mod wire_simulation_scenario_report;
mod test;

fn make_pipeline_with<S>(solver: Result<S, String>) -> Result<PipelinedExecuter<Wiring>, String>
where S: Solve<Wiring> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        WiringParser::new(),
        Ok(TrivialVerifier::new::<Wiring>()),
        solver
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<Wiring>, String> {
    match is_part_2 {
        false => make_pipeline_with(Ok(ZWireNumberCalculator)),
        true => make_pipeline_with(Ok(AssistedAnalyser)),
    }
}


pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(24, false, make_pipeline(false), false)
        ?.try_register(24, true, make_pipeline(true), false)
}