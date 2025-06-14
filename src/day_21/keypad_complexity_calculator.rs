use std::collections::HashMap;

use crate::{answer::{Answer, DisplayableAnswer}, day_21::{caching::DirectionalKeypadChainCacher, keypad::{numerical_keypad, Keypad}, model::Codes, ordered_movement::OrderedMovement}, helper, solver::Solve};

mod error {
    const PREFIX : &str = "[D-21 Keypad complexity calculator]";
    pub fn overflow() -> String {
        format!("{} an overflow occurred during calculation", PREFIX)
    }

    pub fn regex(err: regex::Error) -> String {
        format!("{} could not create solver due to regex error {}", PREFIX, err)
    }

    pub fn number_part(code: &String) -> String {
        format!("{} could not extract number part from code '{}'", PREFIX, code)
    }
}

pub struct KeypadComplexityCalculator {
    scope: usize,
    directional_chain_length: usize,
    keypad: Keypad,
    code_re: regex::Regex,
}

impl KeypadComplexityCalculator {
    pub fn new(scope: usize, directional_chain_length: usize) -> Result<KeypadComplexityCalculator, String> {
        regex::Regex::new(r"^(\d+)A$").map(|code_re|KeypadComplexityCalculator {
            scope, directional_chain_length,
            keypad: numerical_keypad::new(),
            code_re
        }).map_err(error::regex)
    }

    fn sum_complexities(complexities: Vec<u64>) -> Result<u64, String> {
        complexities.into_iter()
            .try_fold(0u64, |acc, complexity|acc.checked_add(complexity))
            .ok_or_else(error::overflow)
    }

    fn number_part(&self, code: &String) -> Result<u64, String> {
        if let Some((_, [number_part])) = self.code_re.captures(code).map(|c|c.extract()) {
            if let Ok(number) = number_part.parse() { return Ok(number) }
        }
        Err(error::number_part(code))
    }

    fn determine_complexity(&self, code: String, cache: &HashMap<OrderedMovement, u64>) -> Result<u64, String> {
        let code_number = match self.number_part(&code) {
            Ok(number) => number,
            Err(message) => return Err(message),
        };

        let mut current_button = 'A';
        let mut click_count : u64 = 0;
        for next_button in code.chars() {

            match self.keypad.get_ordered_movements(current_button, next_button)
                .and_then(|movements|DirectionalKeypadChainCacher::get_cache_min_cost(&movements, cache))
                .and_then(|clicks_required|click_count.checked_add(clicks_required).ok_or_else(error::overflow))
            {
                Ok(new_click_count) => click_count = new_click_count,
                Err(message) => return Err(message),
            }

            current_button = next_button;
        }

        click_count.checked_mul(code_number).ok_or_else(error::overflow)
    }
}


impl Solve<Codes> for KeypadComplexityCalculator {
    fn solve(&self, input: Codes) -> Result<Answer, String> {
        let cache = match DirectionalKeypadChainCacher::new(self.scope).make_caches(self.directional_chain_length) {
            Ok(cache) => cache,
            Err(message) => return Err(message),
        };

        let Codes(codes) = input;
        helper::result::collect(codes.into_iter().map(|code|self.determine_complexity(code, &cache)).collect())
            .and_then(Self::sum_complexities)
            .map(DisplayableAnswer::new)
    }
}