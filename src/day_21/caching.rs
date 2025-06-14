use std::collections::HashMap;

use crate::{day_21::{keypad::{directional_keypad, Keypad}, ordered_movement::OrderedMovement}, helper::movement::Delta};

pub mod error {
    use crate::day_21::ordered_movement::OrderedMovement;

    const PREFIX : &str = "[D-21 Cacher]";

    pub fn not_cached(movement: OrderedMovement) -> String {
        format!("{} movement not cached: {:?}", PREFIX, movement)
    }

    pub fn overflow() -> String {
        format!("{} overflow occurred during cache creation", PREFIX)
    }
}

/// A structure tasked with creating a cache that represents a chain of robots using directional keypads.
/// The cache is of the form, M => C where M is an ordered movement
/// and C is number of clicks needed to faciliate the ordered movement M.
pub struct DirectionalKeypadChainCacher {
    movement_scope: Vec<OrderedMovement>,
    directional_keypad: Keypad,
}

impl DirectionalKeypadChainCacher {

    /// Creates a new cacher for chain of directional keypad with given scope.
    /// The scope represents at most how many steps up/down/left/right will the ordered movements go
    /// when creating caches.
    pub fn new(scope: usize) -> DirectionalKeypadChainCacher {
        DirectionalKeypadChainCacher {
            movement_scope: Self::ordered_movements_cache_scope(scope),
            directional_keypad: directional_keypad::new(),
        }
    }

    /// Makes a cache that represents a chain of robots using directional keypads, where the length
    /// of such chain is `chain_length`.
    pub fn make_caches(&self, chain_length: usize) -> Result<HashMap<OrderedMovement, u64>, String> {
        let mut cache = self.create_user_cache();
        for _ in 0..chain_length {
            match self.create_directional_robot_cache(&cache) {
                Ok(new_cache) => cache = new_cache,
                Err(message) => return Err(message),
            };
        }
        Ok(cache)
    }

    fn ordered_movements_cache_scope(scope: usize) -> Vec<OrderedMovement> {
        let mut rtn = vec![];
        let deltas = (0..scope)
            .flat_map(|num|vec![Delta::Dec(num), Delta::Inc(num)])
            .collect::<Vec<_>>();

        for &first_delta in deltas.iter() {
            for &second_delta in deltas.iter() {
                rtn.push(OrderedMovement::ColRow(first_delta, second_delta));
                rtn.push(OrderedMovement::RowCol(first_delta, second_delta));
            }
        }
        rtn
    }

    fn create_user_cache(&self) -> HashMap<OrderedMovement, u64> {
        let mut base_cache = HashMap::new();
        for &movement in self.movement_scope.iter() {
            let value = movement.get_absolute_change() + 1;
            base_cache.insert(movement, value as u64);
        }
        base_cache
    }


    pub fn get_cache_min_cost(ordered_movements: &Vec<OrderedMovement>, cache: &HashMap<OrderedMovement, u64>) -> Result<u64, String> {
        let mut cache_values = vec![];
        for movement in ordered_movements.iter() {
            if !cache.contains_key(movement) { return Err(error::not_cached(*movement)); }
            cache_values.push(cache[movement]);
        }
        Ok(cache_values.into_iter().min().unwrap())
    }

    fn create_directional_robot_cache(&self, base_cache: &HashMap<OrderedMovement, u64>) -> Result<HashMap<OrderedMovement, u64>, String> {
        let mut cache = HashMap::new();
        for &movement in self.movement_scope.iter() {
            let mut current_button = 'A';
            let mut movement_cost : u64 = 0;
            for next_button in vec![movement.into_buttons(), vec!['A']].concat() {

                match self.directional_keypad.get_ordered_movements(current_button, next_button)
                    .and_then(|movements|Self::get_cache_min_cost(&movements, base_cache))
                    .and_then(|step_cost|step_cost.checked_add(movement_cost).ok_or_else(error::overflow))
                {
                    Ok(new_movement_cost) => movement_cost = new_movement_cost,
                    Err(message) => return Err(message),
                }

                current_button = next_button;
            }
            cache.insert(movement, movement_cost);
        }
        Ok(cache)
    }
}