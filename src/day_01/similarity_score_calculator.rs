use std::collections::HashMap;

use crate::{answer::DisplayableAnswer, solver::Solve};

use super::models::{NumberPair, NumberPairList};

pub struct SimilarityScoreCalculator;

mod error {
    const PREFIX : &str = "[Solver D-01 P2]";

    pub fn solver_overflow_error() -> String {
        format!("{} while calculating similiarity score, an overflow occurred", PREFIX)
    }
}

impl SimilarityScoreCalculator {
    pub fn new() -> SimilarityScoreCalculator { SimilarityScoreCalculator }

    fn create_frequency_map<EXT>(input: &NumberPairList, extractor: EXT) -> HashMap<u32, u32>
    where EXT : Fn(&NumberPair) -> u32 {
        let mut frequency_map = HashMap::new();
        for number in input.list.iter().map(extractor) {
            let freq = frequency_map.get(&number)
                .map_or(0, |f|*f);
            frequency_map.insert(number, freq + 1);
        }
        frequency_map
    }

    fn safe_multiply_add(acc: u32, pair: (u32, u32)) -> Result<u32, String> {
        let (number, freq) = pair;
        number.checked_mul(freq)
            .and_then(|product| acc.checked_add(product))
            .ok_or_else(error::solver_overflow_error)
    }
}

impl Solve<NumberPairList> for SimilarityScoreCalculator {
    fn solve(&self, input: NumberPairList) -> Result<crate::answer::Answer, String> {
        let fmap = Self::create_frequency_map(&input, NumberPair::snd);
        input.list.into_iter()
            .map(|np|np.fst())
            .map(|number| (number,fmap.get(&number).map_or(0, |v|*v)))
            .try_fold(0u32,Self::safe_multiply_add)
            .map(DisplayableAnswer::new)
    }
}