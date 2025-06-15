use std::collections::HashMap;

use crate::{answer::{Answer, DisplayableAnswer}, day_22::{model::Numbers, secret_number_transform::SecretNumberTransform}, helper::movement::Delta, solver::Solve};

/// A finder for the optimal change sequence.
/// 
/// A change sequence is a sequence of fixed length of consecutive price changes,
/// e.g. a sequence of fixed length 3 can be `[+3, -2, +5]` to mean that from
/// some point in time, price was x, then it went up by 3, then down by 2, and lastly up by 5.
/// 
/// A `cutoff` represents the number of secret number evolutions that we consider,
/// ultimately, translating to a length of sequence of prices within which we try to optimise for.
/// 
/// Given the `cutoff` that will translate to length of price sequence and `change_sequence_length`,
/// we work out which change sequence that triggers a transaction will yield the most value (bananas) for us.
pub struct OptimalChangeSequenceFinder {
    cutoff: usize,
    change_sequence_length: usize,
}

impl OptimalChangeSequenceFinder {
    pub fn new(cutoff: usize, change_sequence_length: usize) -> OptimalChangeSequenceFinder {
        OptimalChangeSequenceFinder { cutoff, change_sequence_length }
    }

    /// For the given price sequence, get all change sequences present in the price sequence
    /// and pair up each with its score. A score of the change sequence is the price at the
    /// end of the first occurrence of the change sequence
    /// 
    /// E.g. for a price sequence `[1, 5, 6, 1]` and change sequences of fixed lenght 2, two change
    /// sequences are present in the example price sequence:
    /// - For prefix `[1, 5, 6]` there's a change sequence of `[+4, +1]` with score 6
    /// - For suffix `[5, 6, 1]` there's a change sequence of `[+1, -4]` with score 1
    /// 
    /// Further example, a price sequence `[1, 2, 3, 4, 5, 6, 7]` has only one change sequence of length 2,
    /// and that's `[+1, +1]`. In this case, the first occurrence of this change sequence is corresponding to
    /// prefix of price sequence `[1, 2, 3]` and thus the score is the price that the sequence ends with i.e. 3.
    fn get_change_sequence_scores(&self, price_sequence: Vec<usize>) -> HashMap<Vec<Delta>, u64> {
        let mut scores = HashMap::new();
        for i in self.change_sequence_length..price_sequence.len() {

            // work out change sequence that ends with price at `price_sequence[i]`
            let mut change_sequence = Vec::with_capacity(self.change_sequence_length);
            for previous in i - self.change_sequence_length..i {
                let next = previous + 1;
                let price_change = Delta::infer(price_sequence[previous], price_sequence[next]);
                change_sequence.push(price_change);
            }

            // If this is the first occurrence of the change sequence then its score is `price_sequence[i]`
            if !scores.contains_key(&change_sequence) {
                scores.insert(change_sequence, price_sequence[i] as u64);
            }
        }
        scores        
    }
}

impl Solve<Numbers> for OptimalChangeSequenceFinder {
    fn solve(&self, input: Numbers) -> Result<Answer, String> {
        let Numbers(numbers) = input;
        let transform = SecretNumberTransform::default();
        let mut scores = HashMap::new();

        for number in numbers {
            let price_sequence = transform
                .iterative_evolve_sequence(number, self.cutoff)
                .into_iter()
                .map(|secret_number|(secret_number % 10) as usize)
                .collect();

            // join up existing scores with change sequence scores produced from current price sequence
            for (change_sequence, score) in self.get_change_sequence_scores(price_sequence) {
                let new_score = score + *scores.get(&change_sequence).unwrap_or(&0);
                scores.insert(change_sequence, new_score);
            }
        }
        
        Ok(DisplayableAnswer::new(*scores.values().max().unwrap_or(&0)))
    }
}