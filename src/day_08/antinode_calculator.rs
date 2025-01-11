pub trait AntinodeCalculator {
    /// Given a pair of antennas, returns antinodes produced by them
    fn calculate_antinodes(&self, antennas: ((usize, usize), (usize, usize))) -> Vec<(usize, usize)>;
}