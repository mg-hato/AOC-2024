use std::collections::{HashMap, HashSet};

use crate::{answer::{Answer, DisplayableAnswer}, day_24::{model::Wiring, wire_simulation_scenario::{Scenario, ScenarioGroup}, wires_extractor::WireExtractor, wiring_simulator::SimulationContext}, helper, solver::Solve};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX : &str = "[Assisted Analyser]";

    pub fn bitsize_mismatch(xyz_wires: &[Vec<String>; 3]) -> String {
        vector_display(&vec![
            format!("{} error when verifying extracted wires bit-lengths.", PREFIX),
            format!("X-wires are {} bit(s).", xyz_wires[0].len()),
            format!("Y-wires are {} bit(s).", xyz_wires[1].len()),
            format!("Z-wires are {} bit(s).", xyz_wires[2].len()),
        ], " ")
    }

    pub fn wire_not_initial_wire(wire_type: &str, counterexample_wire: &String) -> String {
        format!("{} all {}-wires must be initial wires. Counterexample: {}",
            PREFIX, wire_type, counterexample_wire)
    }

    pub fn wire_not_gate_output(wire_type: &str, counterexample_wire: &String) -> String {
        format!("{} all {}-wires must be gate-output wires. Counterexample: {}",
            PREFIX, wire_type, counterexample_wire)
    }
}

pub struct AssistedAnalyser;

impl AssistedAnalyser {
    // Extracts wires and verifies them. If successful, returns a triple of X, Y and Z wires (in that order)
    fn extract_wires_and_verify(context: &SimulationContext) -> Result<[Vec<String>; 3], String> {
        let initial_wires = context.get_initial_wire_values().keys().map(|w|w.clone()).collect::<HashSet<_>>();
        let gates = context.get_gates().keys().map(|w|w.clone()).collect::<HashSet<_>>();
        let all_wires = initial_wires.clone().into_iter().chain(gates.clone().into_iter()).collect::<HashSet<_>>();

        let wire_extraction_results = ["x", "y", "z"]
            .map(|prefix|WireExtractor::extract_wires(prefix, &all_wires));

        // if any of the extractions failed, report it
        if wire_extraction_results.iter().any(|extraction_result|extraction_result.is_err()) {
            return Err(helper::result::collect(wire_extraction_results.to_vec()).unwrap_err());
        }
        
        let xyz_wires = wire_extraction_results.map(|extraction_result|extraction_result.unwrap());
        
        // X and Y wires must be of the same bit count. Z wires must have 1 additional bit in comparison.
        if xyz_wires[0].len() != xyz_wires[1].len() || xyz_wires[0].len() + 1 != xyz_wires[2].len() {
            return Err(error::bitsize_mismatch(&xyz_wires));
        }

        // Check all X and Y wires are initial wires
        for (i, &wire_type) in ["x", "y"].iter().enumerate() {
            for wire in xyz_wires[i].iter() {
                if !initial_wires.contains(wire) {
                    return Err(error::wire_not_initial_wire(wire_type, wire));
                }
            }
        }

        // Check all Z wires are gate-output wires
        for wire in xyz_wires[2].iter() {
            if !gates.contains(wire) {
                return Err(error::wire_not_gate_output("z", wire));
            }
        }
        
        Ok(xyz_wires)
    }

    fn make_all_ones_scenario(xyz_wires: &[Vec<String>; 3]) -> Result<ScenarioGroup, ()> {
        let [x_wires, y_wires, z_wires] = xyz_wires;
        if x_wires.len() == 0 {
            return Err(());
        }

        let input = x_wires.iter().chain(y_wires)
            .map(|wire|(wire.clone(), true))
            .collect::<HashMap<String, bool>>();

        let output = z_wires.iter().enumerate()
            .map(|(idx, wire)|(wire.clone(), idx != 0))
            .collect::<HashMap<String, bool>>();

        Ok(ScenarioGroup::new_single(Scenario::new(input, output), format!("All ones in X and Y")))
    }

    fn make_scenario_groups(xyz_wires: &[Vec<String>; 3]) -> Vec<ScenarioGroup> {
        let mut scenario_groups = vec![];

        let [x_wires, y_wires, z_wires] = xyz_wires;

        for current_index in 0..z_wires.len() {
            match ScenarioGroup::make_addition_scenario(x_wires, y_wires, z_wires, current_index) {
                Ok(scenario_group) => scenario_groups.push(scenario_group),
                _ => {},
            }
        }

        if let Ok(all_ones_scenario) = Self::make_all_ones_scenario(xyz_wires) {
            scenario_groups.push(all_ones_scenario);
        }

        scenario_groups
    }

    pub fn all_pass_answer() -> Answer {
        DisplayableAnswer::new("All scenarios have passed successfully")
    } 
}

impl Solve<Wiring> for AssistedAnalyser {
    fn solve(&self, input: Wiring) -> Result<Answer, String> {
        let mut context = match SimulationContext::new(&input, true) {
            Ok(context) => context,
            Err(message) => return Err(message),
        };

        let xyz_wires = match Self::extract_wires_and_verify(&context) {
            Ok(extracted_wires) => extracted_wires,
            Err(message) => return Err(message),
        };

        for scenario_group in Self::make_scenario_groups(&xyz_wires) {
            let scenrio_group_report = match scenario_group.run(&mut context) {
                Ok(res) => res,
                Err(message) => return Err(message),
            };

            if !scenrio_group_report.is_successful() {
                return Ok(Box::new(scenrio_group_report));
            }
        }

        Ok(Self::all_pass_answer())
    }
}