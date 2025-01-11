use crate::helper::table::Table;

use super::{antinode_calculator::AntinodeCalculator, model::AntennaMapField, movement::Movement};

/// An `AntinodeCalculator` implementation that takes into account resonant harmonics,
/// i.e. a pair of antennas will produce antinodes in line with them at the step-distance
/// that is equal to the distance between the two antennas.
pub struct ResonantHarmonicsAntinodeCalculator {
    map: Table<AntennaMapField>,
}

impl ResonantHarmonicsAntinodeCalculator {
    pub fn new(map: Table<AntennaMapField>) -> ResonantHarmonicsAntinodeCalculator {
        ResonantHarmonicsAntinodeCalculator { map }
    }

    fn add_antinodes(&self, start: (usize, usize), movement: Movement, antinodes: &mut Vec<(usize, usize)>) {
        let mut current = Some(start);
        while let Some(pos) = current {
            antinodes.push(pos);
            current = movement.apply(pos).filter(|&position|self.map.get_pos(position).is_some())
        }
    }
}

impl AntinodeCalculator for ResonantHarmonicsAntinodeCalculator {
    fn calculate_antinodes(&self, antennas: ((usize, usize), (usize, usize))) -> Vec<(usize, usize)> {
        let mut antinodes = vec![];
        let (first, second) = antennas;
        self.add_antinodes(first, Movement::infer(second, first), &mut antinodes);
        self.add_antinodes(second, Movement::infer(first, second), &mut antinodes);
        antinodes
    }
}