use std::collections::HashMap;

use crate::day_24::{wire_simulation_scenario_report::ScenarioGroupReport, wiring_simulator::SimulationContext};

pub struct ScenarioGroup {
    scenarios: Vec<Scenario>,
    group_name: String,
}

impl ScenarioGroup {
    pub fn new_single(scenario: Scenario, group_name: String) -> ScenarioGroup {
        ScenarioGroup::new(vec![scenario], group_name)
    }

    fn new(scenarios: Vec<Scenario>, group_name: String) -> ScenarioGroup {
        ScenarioGroup { scenarios, group_name }
    }

    fn choose_n_booleans(n: usize) -> Vec<Vec<bool>> {
        // we start with one trivial boolean sequence: sequence of length zero
        let mut boolean_sequences = vec![vec![]];
        // We want to make all boolean sequences of length `n`.
        // In each iteration of this loop we increase the length
        for _ in 0..n {
            let mut new_boolean_sequences = vec![];

            // take each previous boolean sequence
            for boolean_sequence in boolean_sequences.iter() {
                // extend it with one additional value
                for b in [false, true] {
                    let mut new_boolean_combo = boolean_sequence.clone();
                    new_boolean_combo.push(b);
                    new_boolean_sequences.push(new_boolean_combo);
                }
            }
            boolean_sequences = new_boolean_sequences;
        }
        boolean_sequences
    }

    fn resolve(wires: &Vec<String>, index: usize, negative_offset: usize) -> Option<&String> {
        if index < negative_offset { None }
        else if index - negative_offset < wires.len() {
            Some(&wires[index - negative_offset])
        } else { None }
    }

    fn get_last_three_bit_wires(wires: &Vec<String>, current_index: usize) -> [Option<&String>; 3] {
        [2, 1, 0].map(|negative_offset|Self::resolve(wires, current_index, negative_offset))
    }

    /// Makes addition scenario for bit at `current_index`.
    /// This scenario tests how the result bit reacts to changes in left and right input numbers.
    /// The input numbers will be tweaked only at relevant bits:
    /// - at index `current_index - 2` (if any) will be set to 0 to prevent any carry coming from them
    /// - at index `current_index - 1` (if any) to test for their carry being correctly reflected in resulting bit
    /// - at index `current_index` (if any) to see that the XOR between the two affects the resulting bit
    /// The last two (where appropriate) will take all possible boolean values combinations, i.e. 4 bits with 0 or 1: 16 combinations.
    /// However, it can be the case that `current_index` is 0 and there are no predecessor-bits, in which case this tests for XOR only.
    /// Similarly, another extreme is when we are at the last resulting bit that is only affected by predecessor-bits carry.
    pub fn make_addition_scenario(
        left_wires: &Vec<String>,
        right_wires: &Vec<String>,
        result_wires: &Vec<String>,
        current_index: usize,
    ) -> Result<ScenarioGroup, String> {
        let fail = ||Err(format!("Could not make addition scenario"));
        if current_index >= result_wires.len()
        || current_index >= left_wires.len() + 1
        || current_index >= right_wires.len() + 1 {
            return (fail)();
        }

        let [zero_left, previous_left, current_left] = Self::get_last_three_bit_wires(left_wires, current_index);
        let [zero_right, previous_right, current_right] = Self::get_last_three_bit_wires(right_wires, current_index);

        // wires to be zeroed out in order to not affect addition
        let zeroes = [zero_left, zero_right].iter()
            .filter_map(|&zero|zero)
            .collect::<Vec<_>>();

        // isolated wires that contribute in add operation for the result wire at current index
        let contributors = [previous_left, previous_right, current_left, current_right].iter()
            .filter_map(|&contributor|contributor)
            .collect::<Vec<_>>();

        let mut scenarios = vec![];
        for values in Self::choose_n_booleans(contributors.len()) {
            let mut input = HashMap::new();
            let mut output = HashMap::new();

            // add all contributors
            for (&contributor, &value) in contributors.iter().zip(values.iter()) {
                input.insert(contributor.clone(), value);
            }

            // zero out all zero wires
            for &zero_wire in zeroes.iter() {
                input.insert(zero_wire.clone(), false);
            }

            // helper function to extract input-assigned values
            let get = |opt_wire: Option<&String>|opt_wire
                .and_then(|wire|input.get(wire))
                .map(|b|*b)
                .unwrap_or(false);

            // calculate expected result
            let carry = (get)(previous_left) && (get)(previous_right);
            let result_value = carry ^ (get)(current_left) ^ (get)(current_right);
            output.insert(result_wires[current_index].clone(), result_value);

            scenarios.push(Scenario::new(input, output));
        }

        Ok(ScenarioGroup::new(scenarios, format!("Addition into {}", result_wires[current_index])))
    }

    pub fn run(&self, context: &mut SimulationContext) -> Result<ScenarioGroupReport, String> {
        let mut scenario_results = vec![];
        for scenario in self.scenarios.iter() {
            // if application of scenario caused an error, return it
            if let Err(message) = scenario.apply(context) { return Err(message); }

            match context.simulate() {
                Ok(simulation_values) => scenario_results.push((scenario.clone(), simulation_values)),
                Err(message) => return Err(message)
            };

            // restore context
            context.restore_initial_values()
        }
        let report = ScenarioGroupReport::new(scenario_results, self.group_name.clone());
        Ok(report)
    }
}


#[derive(Debug, Clone)]
pub struct Scenario {
    input_overrides: HashMap<String, bool>,
    expected_outcomes: HashMap<String, bool>,
}

// Something like this
impl Scenario {
    pub fn new(input_overrides: HashMap<String, bool>, expected_outcomes: HashMap<String, bool>) -> Scenario {
        Scenario { input_overrides, expected_outcomes }
    }

    pub fn apply(&self, context: &mut SimulationContext) -> Result<(), String> {
        context.override_initial_values(&self.input_overrides)
    }

    pub fn validate(&self, wire_values: &HashMap<String, bool>) -> bool {
        for (wire, &expected_value) in self.expected_outcomes.iter() {
            if wire_values.contains_key(wire) && *wire_values.get(wire).unwrap() != expected_value {
                return false;
            }
        }
        true
    }

    
    // getters

    pub fn get_expected_outcomes(&self) -> &HashMap<String, bool> { &self.expected_outcomes }

    pub fn get_input_overrides(&self) -> &HashMap<String, bool> { &self.input_overrides}
}