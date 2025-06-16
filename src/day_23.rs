// use crate::{day_22::{model::Numbers, numbers_parser::NumbersParser, optimal_change_sequence_finder::OptimalChangeSequenceFinder, secret_number_examiner::SecretNumberExaminer}, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

use crate::{day_23::{lan_party_password_finder::LanPartyPasswordFinder, local_network_parser::LocalNetworkParser, model::LocalNetwork, triple_connection_detector::TripleConnectionDetector}, executer_manager::ExecuterManager, pipelined_executer::{try_make_pipeline, PipelinedExecuter}, reading::SanitisedFileReader, solver::Solve, verifier::TrivialVerifier};

mod model;
mod local_network_parser;
mod local_network_graph;
mod triple_connection_detector;
mod lan_party_password_finder;
mod mesh_finder;
mod test;

fn make_pipeline_with<S>(solver: S) -> Result<PipelinedExecuter<LocalNetwork>, String>
where S: Solve<LocalNetwork> + 'static {
    try_make_pipeline(
        Ok(SanitisedFileReader::default()),
        LocalNetworkParser::new(),
        Ok(TrivialVerifier::new::<LocalNetwork>()),
        Ok(solver)
    )
}

fn make_pipeline(is_part_2: bool) -> Result<PipelinedExecuter<LocalNetwork>, String> {
    
    match is_part_2 {
        false => make_pipeline_with(TripleConnectionDetector),
        true => make_pipeline_with(LanPartyPasswordFinder),
    }
}


pub fn register(manager: ExecuterManager) -> Result<ExecuterManager, String> {
    manager.try_register(23, false, make_pipeline(false), false)
        ?.try_register(23, true, make_pipeline(true), false)
}