use crate::{day_24::model::{Gate, InitialWire, LogicGate, Wiring}, parser::Parse, reader::{Line, VecLine}};


mod error {
    const PREFIX: &str = "[D-24 parser]";

    pub fn new_regex(purpose: &str, err: regex::Error) -> String {
        format!("{} failed to create regex for {} due to error {}", PREFIX, purpose, err)
    }

    pub fn input_match(line_num: usize) -> String {
        format!("{} failed to match the input line #{}. Line did not match neither initial wire nor gate syntax",
            PREFIX, line_num)
    }
}

pub struct WiringParser{
    initial_wire_re: regex::Regex,
    gate_re: regex::Regex,    
}

impl WiringParser {
    pub fn new() -> Result<WiringParser, String> {
        let initial_wire_regex = regex::Regex::new(r"^([a-z0-9]+) *: *(1|0)$");
        let gate_regex = regex::Regex::new(r"^([a-z0-9]+) +(AND|OR|XOR) +([a-z0-9]+) +-> +([a-z0-9]+)$");
        match (initial_wire_regex, gate_regex) {
            (Err(err), _) => Err(error::new_regex("initial wires", err)),
            (_, Err(err)) => Err(error::new_regex("gates", err)),
            (Ok(initial_wire_re), Ok(gate_re)) => Ok(WiringParser { initial_wire_re, gate_re })
        }
    }

    fn try_parse_wire(&self, line: &Line) -> Result<InitialWire, ()> {
        match self.initial_wire_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [wire_name, value])) => Ok(InitialWire(wire_name.to_owned(), value == "1")),
            None => Err(()),
        }
    }

    fn parse_gate(&self, line: &Line) -> Result<Gate, String> {
        match self.gate_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [left, gate, right, out])) => Ok(Gate(
                left.to_owned(),
                right.to_owned(),
                match gate {
                    "AND" => LogicGate::AND,
                    "OR" => LogicGate::OR,
                    _ => LogicGate::XOR,
                },
                out.to_owned())),
            None => Err(error::input_match(line.number)),
        }
    }
}

impl Parse<Wiring> for WiringParser {
    fn parse(&self, vec_line: VecLine) -> Result<Wiring, String> {
        let mut try_match_initial_wire = true;
        let mut initial_wires = vec![];
        let mut gates = vec![];
        for line in vec_line.lines {
            if try_match_initial_wire {
                match self.try_parse_wire(&line) {
                    Ok(initial_wire) => initial_wires.push(initial_wire),
                    Err(_) => try_match_initial_wire = false,
                };
            }

            if !try_match_initial_wire {
                match self.parse_gate(&line) {
                    Ok(gate) => gates.push(gate),
                    Err(message) => return Err(message),
                }
            };
        }
        Ok(Wiring(initial_wires, gates))
    }
}