use std::collections::HashMap;

use crate::{answer::{Answer, DisplayableAnswer}, helper::result::zip, solver::Solve};

use super::model::Stones;

mod error {
    const PREFIX: &str = "[Solver D-11]";

    pub fn overflow_error(operation: &str, lhs: u64, rhs: u64) -> String {
        format!("{} an overflow occurred during {} of {} and {}", PREFIX, operation, lhs, rhs)
    }
}

pub struct StonePredictionModel {
    blinks: usize,
}

impl StonePredictionModel {
    pub fn new(blinks: usize) -> StonePredictionModel {
        StonePredictionModel { blinks }
    }


    fn add(hashmap: &mut HashMap<u64, u64>, key: u64, increase: u64) -> Result<(), String> {
        if !hashmap.contains_key(&key) {
            hashmap.insert(key, increase);
        } else {
            let old_value = hashmap.get(&key).unwrap();
            let new_value = old_value.checked_add(increase);
            if new_value.is_none() {
                return Err(error::overflow_error("addition", *old_value, increase));
            }
            hashmap.insert(key, new_value.unwrap());
        }
        Ok(())
    }

    /// Given a sequence of initial stones transforms them into a mapping of form `S -> C` that means that stone
    /// with number `S` appears `C` times. This form is called a state in the process of stones' blink transforms.
    fn make_start_state(stones: Stones) -> Result<HashMap<u64, u64>, String> {
        let Stones(stones) = stones;
        let mut state = HashMap::new();
        for stone in stones {
            if let Err(e) = Self::add(&mut state, stone, 1) {
                return Err(e);
            }
        }
        Ok(state)
    } 

    /// Given a state of stones produces a next state corresponding to one-blink transformation.
    fn next(state: HashMap<u64, u64>) -> Result<HashMap<u64, u64>, String> {
        let mut next = HashMap::new();
        for (stone, quantity) in state {
            let outcome = if stone == 0 {
                Self::add(&mut next, 1, quantity)
            } else if stone.to_string().len() % 2 == 0 {
                // work out split into two numbers
                let stone_str = stone.to_string();
                let mid = stone_str.len() / 2;
                let left = stone_str[..mid].parse().unwrap();
                let right = stone_str[mid..].parse().unwrap();

                zip(Self::add(&mut next, left, quantity), Self::add(&mut next, right, quantity), |_,_|())
            } else {
                match stone.checked_mul(2024) {
                    Some(v) => Self::add(&mut next, v, quantity),
                    None => Err(error::overflow_error("multiplication", stone, 2024)),
                }
            };

            if let Err(e) = outcome { return Err(e); }
        }
        Ok(next)
    }

    /// Calculates the number of stones in the given state
    fn stone_count(state: HashMap<u64, u64>) -> Result<u64, String> {
        state.values()
            .try_fold(0u64, |acc, count|acc.checked_add(*count)
                .ok_or_else(||error::overflow_error("stone count sum step", acc, *count)))
    }
}

impl Solve<Stones> for StonePredictionModel {
    fn solve(&self, input: Stones) -> Result<Answer, String> {
        let mut i = 0;
        let mut state = Self::make_start_state(input);
        while i < self.blinks {
            state = match state {
                Ok(current_state) => Self::next(current_state),
                Err(e) => return Err(e),
            };
            i += 1;
        }
        state.and_then(Self::stone_count).map(DisplayableAnswer::new)
    }
}