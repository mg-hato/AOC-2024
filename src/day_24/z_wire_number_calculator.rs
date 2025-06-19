use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, day_24::{model::Wiring, wires_extractor::WireExtractor, wiring_simulator::{SimulationContext}}, helper, solver::Solve};

pub struct ZWireNumberCalculator;

impl Solve<Wiring> for ZWireNumberCalculator {
    fn solve(&self, input: Wiring) -> Result<Answer, String> {
        let context = match SimulationContext::new(&input, false) {
            Ok(context) => context,
            Err(message) => return Err(message),
        };
        
        let all_wires = context.get_initial_wire_values().keys().chain(context.get_gates().keys())
            .map(|wire|wire.clone())
            .collect::<HashSet<_>>();
        
        helper::result::zip(
            WireExtractor::extract_wires("z", &all_wires),
            context.simulate(),
            |z_wires, results|WireExtractor::try_make_number(&z_wires, &results)
        ).and_then(|r|r).map(DisplayableAnswer::new)
    }
}