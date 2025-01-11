use crate::helper::table::Table;

use super::{antinode_calculator::AntinodeCalculator, model::AntennaMapField, movement::Movement};

/// A simple `AntinodeCalculator` implementation where a pair of antennas will produce
/// exactly two antinodes on the line that crosses through the pair on each end
pub struct SimpleAntinodeCalculator {
    map: Table<AntennaMapField>,
}
impl SimpleAntinodeCalculator {
    pub fn new(map: Table<AntennaMapField>) -> SimpleAntinodeCalculator {
        SimpleAntinodeCalculator { map }
    }
}

impl AntinodeCalculator for SimpleAntinodeCalculator {
    fn calculate_antinodes(&self, antennas: ((usize, usize), (usize, usize))) -> Vec<(usize, usize)> {
        let (first, second) = antennas;
        [
            Movement::infer(first, second).apply(second),
            Movement::infer(second, first).apply(first),
        ]   
            .iter()
            .map_while(|&position|position)
            .filter(|&position|self.map.get_pos(position).is_some())
            .collect()
    }
}
