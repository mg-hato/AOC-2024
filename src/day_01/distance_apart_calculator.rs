
use crate::{answer::DisplayableAnswer, solver::Solve};

use super::models::{NumberPair, NumberPairList};


pub struct DistanceApartCalculator;

mod error {
    const PREFIX : &str = "[Solver D-01 P1]";

    pub fn solver_overflow_error() -> String {
        format!("{} while summing up, an overflow occurred", PREFIX)
    }
}

impl DistanceApartCalculator {
    pub fn new() -> DistanceApartCalculator { DistanceApartCalculator }

    fn extract_sort<EXT>(input: &NumberPairList, extractor: EXT) -> Vec<u32>
    where EXT : Fn(&NumberPair) -> u32 {
        let mut extracted : Vec<u32> = input.list.iter().map(extractor).collect();
        extracted.sort();
        extracted
    }

    fn add_pair_distance(acc : u32, pair : (u32, u32)) -> Result<u32, String> {
        let (lhs, rhs) = pair;
        acc.checked_add(lhs.abs_diff(rhs))
            .ok_or_else(||error::solver_overflow_error())
    }
}

impl Solve<NumberPairList> for DistanceApartCalculator {
    fn solve(&self, input: NumberPairList) -> Result<crate::answer::Answer, String> {
        Self::extract_sort(&input, NumberPair::fst).into_iter()
            .zip(Self::extract_sort(&input, NumberPair::snd).into_iter())
            .try_fold(0u32, Self::add_pair_distance)
            .map(DisplayableAnswer::new)
    }
}