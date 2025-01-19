use crate::helper::position::UPosition;

pub trait AntinodeCalculator {
    /// Given a pair of antennas, returns antinodes produced by them
    fn calculate_antinodes(&self, antennas: (UPosition, UPosition)) -> Vec<UPosition>;
}