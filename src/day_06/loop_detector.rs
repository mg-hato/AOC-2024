use std::collections::HashSet;

use super::{guard_state::GuardState, next_state::NextState};

pub trait LoopDetector {
    fn next_state(&mut self, current_state: GuardState) -> Result<NextState, String>;
    fn starting_state(&self) -> GuardState;
}

/// Returns a result of bool s.t. returned bool is `true` if and only if the guard
/// will end up in a closed loop given their starting state. If any unexpected error
/// occurs, an error is returned instead of the boolean.
/// 
/// An error can occur from a bad starting state for example.
pub fn loops<LD>(detector: &mut LD) -> Result<bool, String>
where LD: LoopDetector {
    let mut visited_states = HashSet::new();
    let mut next_state = NextState::Next(detector.starting_state());
    while let NextState::Next(state) = next_state {

        // If `insert` returns false, it means the state is already visited
        // In that case, we have a loop!
        if !visited_states.insert(state) { return Ok(true); }
        
        match detector.next_state(state) {
            Ok(next) => { next_state = next; },
            Err(error) => return Err(error),
        }
    }
    // We left the while loop, meaning the next state must have been the `Out` state
    // Therefore, guard does not end up in a closed loop, hence return false
    Ok(false)
}