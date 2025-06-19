use std::collections::{HashMap, HashSet};

use crate::day_24::model::{Gate, InitialWire, Wiring};


#[derive(Clone)]
pub struct SimulationContext {
    wire_to_initial_value: HashMap<String, bool>,

    /// A mapping from an output wire to its gate
    wire_to_gate: HashMap<String, Gate>,

    topological_order: Vec<String>,

    original_wire_to_initial_value: HashMap<String, bool>
}

impl SimulationContext {

    pub fn new(wiring: &Wiring, expect_dag: bool) -> Result<SimulationContext, String> {
        let Wiring(initial_wires, gates) = wiring;
        
        let initial_wires = match Self::try_process_initial_wires(initial_wires) {
            Ok(initial_wires) => initial_wires,
            Err(message) => return Err(message),
        };

        let gates = match Self::try_process_gates(&initial_wires, gates) {
            Ok(gates) => gates,
            Err(message) => return Err(message),
        };

        match Self::try_topological_sort(&initial_wires, &gates, expect_dag) {
            Ok(topological_order) => Ok(SimulationContext {
                original_wire_to_initial_value: initial_wires.clone(),
                wire_to_initial_value: initial_wires,
                wire_to_gate: gates,
                topological_order,
            }),
            Err(message) => Err(message),
        }
    }

    fn try_process_initial_wires(initial_wires: &Vec<InitialWire>) -> Result<HashMap<String, bool>, String> {
        let mut inital_wires_values = HashMap::new();
        for InitialWire(wire, initial_value) in initial_wires.clone() {
            if let Some(&value) = inital_wires_values.get(&wire) {
                if value != initial_value {
                    return Err(format!("Distinct initial wire values for wire {}", wire));
                }
            }
            inital_wires_values.insert(wire, initial_value);
        }
        Ok(inital_wires_values)
    }

    fn try_process_gates(initial_wires: &HashMap<String, bool>, gates: &Vec<Gate>)
        -> Result<HashMap<String, Gate>, String>
    {
        let mut gate_mapping = HashMap::new();
        for gate in gates.iter() {
            let Gate(_, _, _, output_wire) = gate;
            if initial_wires.contains_key(output_wire) {
                return Err(format!("Initial wire {} cannot also be an output wire", output_wire));
            } else if gate_mapping.contains_key(output_wire) {
                return Err(format!("Output wire {} is an output wire of more than one gate", output_wire));
            }
            gate_mapping.insert(output_wire.clone(), gate.clone());
        }
        Ok(gate_mapping)
    }

    fn try_topological_sort(initial_wires: &HashMap<String, bool>, gates: &HashMap<String, Gate>, expect_dag: bool)
        -> Result<Vec<String>, String>
    {
        // create edges: input_wire => list of dependant output wires
        let edges = {
            let mut edges = HashMap::new();
            for Gate(left, right, _, output_wire) in gates.values() {
                for input_wire in [left, right] {
                    if !edges.contains_key(input_wire) {
                        edges.insert(input_wire.clone(), vec![]);
                    }
                    edges.get_mut(input_wire).unwrap().push(output_wire.clone());
                }
            }
            edges
        };

        // create initial queue: all wires with initial values
        let mut queue = {
            let mut queue = vec![];
            initial_wires.keys().for_each(|wire|queue.push(wire.clone()));
            queue
        };

        let mut topological_order = vec![];
        let mut wires_done = HashSet::new();
        while !queue.is_empty() {
            let wire = queue.pop().unwrap();
            wires_done.insert(wire.clone());
            for dependant_wire in edges.get(&wire).unwrap_or(&vec![]).iter() {
                let Gate(left, right, _, _) = gates.get(dependant_wire).unwrap();
                if [left, right].iter().all(|&input_wire|wires_done.contains(input_wire)) {
                    queue.push(dependant_wire.clone());
                }
            }
            topological_order.push(wire);
        }
        if expect_dag && topological_order.len() != gates.len() + initial_wires.len() {
            println!("TOP: {}; GATES: {}; IW: {}", topological_order.len(), gates.len(), initial_wires.len());
            Err(format!("Wiring does not form a DAG"))
            // PRODUCE A DAG REPORT
        } else {
            Ok(topological_order)
        }
    }

    pub fn resolve_value(&self, wire: &String, wire_values: &HashMap<String, bool>) -> Option<bool> {
        if let Some(&value) = self.wire_to_initial_value.get(wire) {
            return Some(value);
        }

        self.wire_to_gate.get(wire)
            .filter(|Gate(left, right, _, _)|[left, right].iter().all(|&w|wire_values.contains_key(w)))
            .map(|Gate(left, right, logic_gate, __)|{
                let (left_value, right_value) = (wire_values[left], wire_values[right]);
                match logic_gate {
                    super::model::LogicGate::XOR => left_value ^ right_value,
                    super::model::LogicGate::AND => left_value && right_value,
                    super::model::LogicGate::OR => left_value || right_value,
                }
            })
    }

    pub fn simulate(&self) -> Result<HashMap<String, bool>, String> {
        let mut results = HashMap::new();
        for wire in self.topological_order.iter() {
            if let Some(value) = self.resolve_value(wire, &results) {
                results.insert(wire.clone(), value);
            } else {
                return Err(format!("Could not resolve the value for wire {}", wire))
            }
        }
        Ok(results)
    }

    pub fn override_initial_values(&mut self, overrides: &HashMap<String, bool>) -> Result<(), String> {
        if let Err(message) = overrides.keys()
            .try_fold((), |_, wire|match self.wire_to_initial_value.contains_key(wire) {
                true => Ok(()),
                false => Err(format!("Cannot override wire value {} as it is not an initial wire", wire))
        }) { Err(message) } else {
            Self::override_values(&mut self.wire_to_initial_value, overrides);
            Ok(())
        }
    }

    pub fn restore_initial_values(&mut self) {
        Self::override_values(&mut self.wire_to_initial_value, &self.original_wire_to_initial_value);
    }

    fn override_values(target: &mut HashMap<String, bool>, override_map: &HashMap<String, bool>) {
        override_map.iter().for_each(|(wire, value)|*target.get_mut(wire).unwrap() = *value);
    }

    pub fn get_initial_wire_values(&self) -> HashMap<String, bool> { self.wire_to_initial_value.clone() }
    pub fn get_gates(&self) -> HashMap<String, Gate> { self.wire_to_gate.clone() }
}