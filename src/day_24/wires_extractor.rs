use std::collections::{HashMap, HashSet};

mod error {
    use std::num::ParseIntError;

    const PREFIX: &str = "[D-24 wire extractor]";

    pub fn wire_regex(wire_prefix: &String, err: regex::Error) -> String {
        format!("{} could not create the {}-wire regex due to error {}", PREFIX, wire_prefix, err)
    }

    pub fn wire_number_parse(wire_prefix: &String, num_part: &str, err: ParseIntError) -> String {
        format!("{} could not parse a {}-wire number '{}' because of parsing error {}", PREFIX, wire_prefix, num_part, err)
    }

    pub fn missing_bit_value(wire: &String, index: usize) -> String {
        format!("{} could not create number from sequence of wires. Missing bit value at index #{} for wire '{}'",
            PREFIX, index, wire)
    }

    pub fn overflow(msb_index: usize) -> String {
        format!("{} could not make a number from sequence of wires as its most significant bit is at index {}",
            PREFIX, msb_index)
    }

    pub fn duplicates(wire_prefix: &String) -> String {
        format!("{} duplicate {}-wire", PREFIX, wire_prefix)
    }

    pub fn no_matching_wires(wire_prefix: &String) -> String {
        format!("{} no matching {}-wires", PREFIX, wire_prefix)
    }

    pub fn missing_wire(wire_prefix: &String, i: u8) -> String {
        format!("{} missing {}-wire at index {}. No continuous {}-wire number can be formed",
            PREFIX, wire_prefix, i, wire_prefix)
    }
}

pub struct WireExtractor {
    wire_prefix: String,
    wire_re: regex::Regex
}

impl WireExtractor {
    pub fn new(prefix: &str) -> Result<WireExtractor, String> {
        let wire_prefix = prefix.to_string();
        regex::Regex::new(format!("^{}(\\d+)$", prefix).as_str())
            .map_err(|err|error::wire_regex(&wire_prefix, err))
            .map(|wire_re|WireExtractor { wire_re, wire_prefix })
    }

    /// Try get a number of the wire. Returns:
    /// - `Ok(Some(i))` if the passed wire matches the wire RE and the suffix indicates it is `i`-th wire
    /// - `Ok(None)` if the passed wire does not match wire RE.
    /// - `Err(message)` if the passed wire matches the wire RE but the suffix number has an issue (parsing/out of bound).
    fn try_get_wire_number(&self, wire: &String) -> Result<Option<u8>, String> {
        match self.wire_re.captures(wire).map(|c|c.extract()) {
            Some((_, [number_part])) => number_part.parse()
                .map_err(|err|error::wire_number_parse(&self.wire_prefix, number_part, err))
                .map(|number|Some(number)),
            None => Ok(None),
        }
    }

    /// From wires provided tries to form a continuous sequence of matching wires, starting with 0-th wire
    /// all the way up to the maximum detected wire.
    /// If returned `Ok` result, it is a vector such that at index `i` the `i`-th wire name is contained.
    pub fn get_matching_wires(&self, all_wires: &HashSet<String>) -> Result<Vec<String>, String> {
        let mut matching_wires = HashMap::new();

        // go through all wires, keep the matching ones; report any errors
        for wire in all_wires.iter() {
            match self.try_get_wire_number(wire) {
                Ok(Some(i)) if matching_wires.contains_key(&i) => return Err(error::duplicates(&self.wire_prefix)),
                Ok(Some(i)) => { matching_wires.insert(i, wire.clone()); }
                Err(message) => return Err(message),
                _ => { },
            }
        }
        let matching_wires = matching_wires;

        // check if any wires are matching
        if matching_wires.is_empty() {
            return Err(error::no_matching_wires(&self.wire_prefix));
        }

        // try form a continuous uninterrupted sequence of matching wires
        let maximum_index = *matching_wires.keys().max().unwrap();
        let mut matching_wire_sequence = vec![];
        for index in 0..maximum_index + 1 {
            match matching_wires.get(&index) {
                Some(wire) => matching_wire_sequence.push(wire.clone()),
                None => return Err(error::missing_wire(&self.wire_prefix, index)),
            }
        }
        Ok(matching_wire_sequence)
    }

    // pub fn process(&self, info: &WiringInfo) -> Result<HashSet<String>, String> {
    //     let mut x = HashSet::new();
    //     for wire in info.get_all_wires() {
    //         match self.try_get_wire_number(&wire) {
    //             Ok(Some(x)) => {

    //             },
    //             Err(message) => return Err(message),
    //         }
    //         _ => {},
    //     };
        // let ddx = collect(info.get_all_wires().iter()
        //     .map(|wire|self.try_get_wire_number(wire))
        //     .filter(|result|result.is_err() || result.is_ok_and(|n|n.is_some()))
        //     .collect());
        // self.try_get_wire_number(wire)
    // }

    pub fn extract_wires(prefix: &str, all_wires: &HashSet<String>) -> Result<Vec<String>, String> {
        Self::new(prefix)
            .and_then(|extractor|extractor.get_matching_wires(all_wires))
    }

    pub fn try_make_number(component_wires: &Vec<String>, wire_values: &HashMap<String, bool>) -> Result<u64, String> {
        
        if component_wires.len() > 64 { return Err(error::overflow(component_wires.len() - 1)); }
        
        let mut number : u64 = 0;
        let mut power : u64 = 0;
        for (index, wire) in component_wires.iter().enumerate() {
            power = if power == 0 { 1 } else { power * 2 };

            if !wire_values.contains_key(wire) {
                return Err(error::missing_bit_value(wire, index));
            }
            else if wire_values[wire] {
                number += power;
            }
        }
        Ok(number)
    }
}