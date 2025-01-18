use crate::helper::{boundary::apply, movement::Movement, position::UPosition, table::{Table, TableBound}};

use super::{antinode_calculator::AntinodeCalculator, model::AntennaMapField};

/// An `AntinodeCalculator` implementation that takes into account resonant harmonics,
/// i.e. a pair of antennas will produce antinodes in line with them at the step-distance
/// that is equal to the distance between the two antennas.
pub struct ResonantHarmonicsAntinodeCalculator {
    boundaries: TableBound,
}

impl ResonantHarmonicsAntinodeCalculator {
    pub fn new(map: Table<AntennaMapField>) -> ResonantHarmonicsAntinodeCalculator {
        ResonantHarmonicsAntinodeCalculator { boundaries: map.boundary() }
    }

    fn add_antinodes(&self, start: UPosition, movement: Movement, antinodes: &mut Vec<UPosition>) {
        let mut current = Some(start);
        while let Some(pos) = current {
            antinodes.push(pos);
            current = apply(self.boundaries, movement, pos);
        }
    }
}

impl AntinodeCalculator for ResonantHarmonicsAntinodeCalculator {
    fn calculate_antinodes(&self, antennas: (UPosition, UPosition)) -> Vec<UPosition> {
        let mut antinodes = vec![];
        let (first, second) = antennas;
        self.add_antinodes(first, Movement::infer(second, first), &mut antinodes);
        self.add_antinodes(second, Movement::infer(first, second), &mut antinodes);
        antinodes
    }
}