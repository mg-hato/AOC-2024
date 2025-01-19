use crate::helper::{boundary::apply, movement::Movement, position::UPosition, table::{Table, TableBound}};

use super::{antinode_calculator::AntinodeCalculator, model::AntennaMapField};

/// A simple `AntinodeCalculator` implementation where a pair of antennas will produce
/// exactly two antinodes on the line that crosses through the pair on each end
pub struct SimpleAntinodeCalculator {
    boundaries: TableBound,
}
impl SimpleAntinodeCalculator {
    pub fn new(map: Table<AntennaMapField>) -> SimpleAntinodeCalculator {
        SimpleAntinodeCalculator { boundaries: map.boundary() }
    }
}

impl AntinodeCalculator for SimpleAntinodeCalculator {
    fn calculate_antinodes(&self, antennas: (UPosition, UPosition)) -> Vec<UPosition> {
        let (first, second) = antennas;
        let movement = Movement::infer(first, second);
        [
            apply(self.boundaries, movement, second),
            apply(self.boundaries, movement.inverse(), first),
        ].iter().filter_map(|&x|x).collect()
    }
}
