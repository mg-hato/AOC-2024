use std::collections::HashMap;
use crate::{answer::Report, day_24::wire_simulation_scenario::Scenario};

#[derive(Debug)]
pub struct ScenarioGroupReport {
    scenario_results: Vec<(Scenario, HashMap<String, bool>)>,
    group_name: String,
}

impl ScenarioGroupReport {
    pub fn new(scenario_results: Vec<(Scenario, HashMap<String, bool>)>, group_name: String) -> ScenarioGroupReport {
        ScenarioGroupReport { scenario_results, group_name }
    }

    const TAB : &str = "  ";
    const NEWLINE : &str = "\n";

    fn tab(builder: &mut string_builder::Builder, tab_level: usize) {
        for _ in 0..tab_level {
            builder.append(Self::TAB);
        }
    }

    fn list_key_value_map<V, V2SFN>(
        builder: &mut string_builder::Builder,
        values: &HashMap<String, V>,
        tab_level: usize,
        value_to_string_fn: V2SFN,
    ) where V2SFN : Fn(&V) -> String {
        if values.len() == 0 {
            builder.append(Self::NEWLINE);
            Self::tab(builder, tab_level);
            builder.append("- <empty>");
            return;
        }

        for (key, value) in values.iter() {
            builder.append(Self::NEWLINE);
            Self::tab(builder, tab_level);
            builder.append(format!("- {} : {}", key, (value_to_string_fn)(value)));
        }
    }

    fn get_differences(expected: &HashMap<String, bool>, actual: &HashMap<String, bool>) -> HashMap<String, Option<bool>> {
        let mut differences = HashMap::new();
        for (wire, &expected_value) in expected {
            let actual_value = actual.get(wire).map(|b|*b);
            if actual_value.is_none_or(|act_value|act_value != expected_value) {
                differences.insert(wire.clone(), actual_value);
            }
        }
        differences
    }

    pub fn is_successful(&self) -> bool {
        self.scenario_results.iter().all(|(scenario, values)|scenario.validate(values))
    }

    fn bool_to_string(value: &bool) -> String {
        format!("{}", if *value { 1 } else { 0 })
    }

    fn optional_bool_to_string(value: &Option<bool>) -> String {
        match *value {
            Some(boolean) => Self::bool_to_string(&boolean),
            None => format!("X"),
        }
    }
}

impl Report for ScenarioGroupReport {
    fn report(&self) -> String {
        let mut builder = string_builder::Builder::default();
        builder.append(format!("Scenario group '{}' results:", self.group_name));
        
        for (i, (scenario, values)) in self.scenario_results.iter().enumerate() {
            builder.append(Self::NEWLINE);
            Self::tab(&mut builder, 1);
            builder.append(format!("- Scenario #{}:", i + 1));

            // do not report in detail if scenario was successful
            if scenario.validate(values) {
                builder.append(" successful");
                continue;
            } else {
                builder.append("unsuccessful");
            }

            // Report on input overrides
            builder.append(Self::NEWLINE);
            Self::tab(&mut builder, 2);
            builder.append("- Input overrides:");
            Self::list_key_value_map(&mut builder, scenario.get_input_overrides(), 3, Self::bool_to_string);

            // Report on expected outcomes
            builder.append(Self::NEWLINE);
            Self::tab(&mut builder, 2);
            builder.append("- Expected outcomes:");
            Self::list_key_value_map(&mut builder, scenario.get_expected_outcomes(), 3, Self::bool_to_string);

            // Report on diffs
            builder.append(Self::NEWLINE);
            Self::tab(&mut builder, 2);
            builder.append("- Differences from actual outcomes:");
            let diffs = Self::get_differences(scenario.get_expected_outcomes(), values);
            Self::list_key_value_map(&mut builder, &diffs, 3, Self::optional_bool_to_string);
        }

        match builder.string() {
            Ok(report_text) => report_text,
            Err(e) => format!("Failed to build the scenario group report due to error: {}", e)
        }
    }
}