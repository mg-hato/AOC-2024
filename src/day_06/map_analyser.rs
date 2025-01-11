use std::collections::{HashMap, HashSet};

use crate::helper::table::Table;

use super::{direction::Direction, guard_state::GuardState, loop_detector::LoopDetector, models::LaboratoryMapField, next_state::NextState};

pub struct MapAnalyser {
    guard_start_position: (usize, usize),

    /// A mapping from position (row, column) -> whether it is free to step into
    free_position_map: HashMap<(usize, usize), bool>, 
}

mod error {
    const PREFIX: &str = "[MapAnalyser]";

    pub fn guard_on_blocked_position_error(position: (usize, usize)) -> String {
        let (row, col) = position;
        format!("{} guard cannot be on a blocked position: ({},{})", PREFIX, row, col)
    }
    
    pub fn starting_guard_position_error() -> String {
        format!("{} exactly one guard starting position is expected", PREFIX)
    }
}

impl MapAnalyser {
    pub fn new(input: Table<LaboratoryMapField>) -> Result<MapAnalyser, String> {
        let mut guard_positions = vec![];
        let mut free_position_map = HashMap::new();

        for (pos, &field) in input.iter() {
            free_position_map.insert(pos, field != LaboratoryMapField::Block);
            if field == LaboratoryMapField::Guard {
                guard_positions.push(pos);
            }
        }

        match guard_positions.len() {
            1 => Ok(MapAnalyser { guard_start_position: guard_positions[0], free_position_map }),
            _ => Err(error::starting_guard_position_error()),
        }
    }
    
    /// Performs analysis on the underlying map, creating and returning a path of states that
    /// the guard follows until exiting the visibile part of the map. If the guard ends up in
    /// a closed loop inside the visible part of the map, the returned path is an empty sequence.
    /// 
    /// In case of an error, it returns it instead of the path of states.
    /// 
    /// Bear in mind that the returned sequence is empty if and only if the guard loops,
    /// because if the guard does not loop, at least the guard's starting state will be
    /// in the returned path. 
    pub fn perform_analysis(&mut self) -> Result<Vec<GuardState>, String> {
        let mut visited_states = HashSet::new();
        let mut state_path = vec![];
        let mut next_state = NextState::Next(self.starting_state());

        while let NextState::Next(state) = next_state {
            if !visited_states.insert(state) { return Ok(vec![]) }
            state_path.push(state);
            match self.next_state(state) {
                Ok(next) => next_state = next,
                Err(error) => return Err(error),
            }
        }
        Ok(state_path)
    }
}

impl LoopDetector for MapAnalyser {
    fn next_state(&mut self, current_state: GuardState) -> Result<NextState, String> {

        // Do a check on the current state position
        match self.free_position_map.get(&current_state.position) {
            Some(is_current_free) => {
                // Guard cannot be on a non-free position
                if !is_current_free {
                    return Err(error::guard_on_blocked_position_error(current_state.position))
                }
            },
            None => return Ok(NextState::Out), // position is already out
        };

        // Work out the next guards state
        Ok(current_state.next_position()
            .and_then(|position|self.free_position_map.get(&position).map(|is_free|match is_free {
                true  => GuardState::new(position, current_state.direction),
                false => current_state.rotate(),
            }))
            .map_or(NextState::Out, NextState::Next))
    }
    
    fn starting_state(&self) -> GuardState { GuardState::new(self.guard_start_position, Direction::Up) }
}