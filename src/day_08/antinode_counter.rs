use std::collections::{HashMap, HashSet};

use crate::{answer::{Answer, DisplayableAnswer}, helper::table::Table, solver::Solve};

use super::{antinode_calculator::AntinodeCalculator, model::AntennaMapField};

/// Antinode counter that counts number of unique antinodes
/// produced by all pairs of antennas of same frequency. 
/// 
/// It has a private member function that given a map of antennas produces an `AntinodeCalculator`.
pub struct AntinodeCounter<AC> where AC: AntinodeCalculator + 'static {
    antinode_calculator_fn: Box<dyn Fn(Table<AntennaMapField>) -> AC>,
}

impl <AC> AntinodeCounter<AC> where AC: AntinodeCalculator + 'static {
    pub fn new<ACF>(antinode_calculator_fn: ACF) -> AntinodeCounter<AC>
    where ACF: Fn(Table<AntennaMapField>) -> AC + 'static {
        AntinodeCounter { antinode_calculator_fn: Box::new(antinode_calculator_fn) }
    }

    /// Creates a mapping `X -> Y` where
    /// - `X` is a frequency that exists on the input map
    /// - `Y` is a collection (`Vec`) of positions that have the frequency of `X` on the map
    fn make_frequency_map(input: &Table<AntennaMapField>) -> HashMap<char, Vec<(usize, usize)>> {
        let mut frequency_map = HashMap::new();
        for (pos, &field) in input.iter() {
            if let AntennaMapField::Antenna(frequency) = field {
                if !frequency_map.contains_key(&frequency) {
                    frequency_map.insert(frequency, vec![]);
                }
                frequency_map.get_mut(&frequency).unwrap().push(pos);
            }
        }
        frequency_map
    }
}

impl <AC> Solve<Table<AntennaMapField>> for AntinodeCounter<AC> where AC: AntinodeCalculator + 'static {
    fn solve(&self, input: Table<AntennaMapField>) -> Result<Answer, String> {

        let frequency_map  = Self::make_frequency_map(&input);
        let antinode_calculator = (self.antinode_calculator_fn)(input);
        let mut antinodes = HashSet::new();
        for (_, antennas) in frequency_map {
            for i in 0..antennas.len() {
                for j in i+1..antennas.len() {

                    antinode_calculator.calculate_antinodes((antennas[i], antennas[j]))
                        .into_iter().for_each(|antinode|{ antinodes.insert(antinode); });

                }
            }
        }
        Ok(DisplayableAnswer::new(antinodes.len()))
    }
}