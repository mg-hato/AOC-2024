use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Wiring(pub Vec<InitialWire>, pub Vec<Gate>);

impl Display for Wiring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Wiring(initial_wires, gates) = self;
        write!(f, "{{ Initial: [{}], Gates: [{}] }}", vector_display(initial_wires, ","), vector_display(gates, ","))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InitialWire(pub String, pub bool);

impl Display for InitialWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let InitialWire(wire, value) = self;
        write!(f, "{}: {}", wire, if *value { 1 } else { 0 })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Gate(pub String, pub String, pub LogicGate, pub String);

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Gate(left_wire, right_wire, logic_gate, out_wire) = self;
        write!(f, "{} {} {} -> {}", left_wire, logic_gate, right_wire, out_wire)
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LogicGate {
    XOR,
    AND,
    OR,
}

impl Display for LogicGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LogicGate::XOR => "XOR",
            LogicGate::AND => "AND",
            LogicGate::OR => "OR",
        })
    }
}