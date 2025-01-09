use std::collections::HashMap;


use super::{guard_state::GuardState, loop_detector::LoopDetector, map_analyser::MapAnalyser, next_state::NextState};


/// A loop detector with an optimised `next_state`.
/// Each next state query jumps over step-by-step states
/// and returns only a state that:
/// - Leaves the map; if no obstacles are on the way
/// - Or the first state where the direction has changed due to an obstacle
pub struct OptimisedCachingLoopDetector<'a> {
    map_analyser: &'a mut MapAnalyser,
    cache: HashMap<GuardState, NextState>,
}

impl OptimisedCachingLoopDetector<'_> {
    pub fn new(map_analyser: &mut MapAnalyser) -> OptimisedCachingLoopDetector {
        OptimisedCachingLoopDetector { map_analyser, cache: HashMap::new() }
    }

    fn populate_cache_with(&mut self, keys: Vec<GuardState>, value: NextState) {
        for key in keys { self.cache.insert(key, value); }
    }

    /// Populates cache for requested state by using underlying map analyser to see
    /// the sequence of states until an appropriate state has been reached.
    /// It also caches for the states in the sequence
    fn populate_cache_for(&mut self, requested: GuardState) -> Result<(), String> {
        let mut keys = vec![]; // Keys that will be cached
        let mut latest_state = Ok(NextState::Next(requested));

        while let Ok(next_state) = latest_state {
            match next_state {

                NextState::Next(state) => {
                    // If directions change, an obstacle is in front
                    if state.direction != requested.direction {
                        self.populate_cache_with(keys, next_state);
                        break;
                    }

                    // If latest state already has associated cached value, use it for others as well
                    else if let Some(&cached_value) = self.cache.get(&state) {
                        self.populate_cache_with(keys, cached_value);
                        break;
                    }

                    // otherwise, add latest state in keys to be cached and move on
                    keys.push(state);
                    latest_state = self.map_analyser.next_state(state);
                },

                NextState::Out => {
                    self.populate_cache_with(keys, NextState::Out);
                    break;
                },
            }
        }
        latest_state.map(|_|())
    }
}

impl <'a> LoopDetector for OptimisedCachingLoopDetector<'a> {
    fn next_state(&mut self, current_state: GuardState) -> Result<NextState, String> {
        if !self.cache.contains_key(&current_state) {
            // Populate cache for requested state, but handle potential error
            if let Err(error) = self.populate_cache_for(current_state) {
                return Err(error);
            }
        }

        Ok(self.cache[&current_state])
    }

    fn starting_state(&self) -> GuardState {
        self.map_analyser.starting_state()
    }
}

