use std::collections::{HashMap, HashSet, VecDeque};

use crate::{answer::{Answer, DisplayableAnswer}, helper::{movement::{self}, position::UPosition}, solver::Solve};

use super::model::{BytePosition, FallingBytes};

mod error {
    use crate::{day_18::model::BytePosition, helper::position::UPosition};

    const PREFIX: &str = "[Solver D-18 Part 1]";

    pub fn byte_out_of_bounds(index: usize, bp: BytePosition, corner: UPosition) -> String {
        format!("{} byte {} at index {} is out of bounds. Bottom-right corner: {}", PREFIX, bp, index, corner)
    }

    pub fn start_out_of_bounds(start: UPosition, corner: UPosition) -> String {
        format!("{} start position {} is out of bounds. Bottom-right corner: {}", PREFIX, start, corner)
    }

    pub fn unreachable(goal: UPosition) -> String {
        format!("{} goal position {} is unreachable", PREFIX, goal)
    }
}
pub struct MemorySpacePathFinder {
    bottom_right_corner: UPosition,
    simulate_first: usize,
}

/// State used for Breadth-First-Search
#[derive(Clone, Copy)]
struct State {
    pos: UPosition,
    steps: usize,
}

impl MemorySpacePathFinder {
    pub fn new(bottom_right_corner: UPosition, simulate_first: usize) -> MemorySpacePathFinder {
        MemorySpacePathFinder { bottom_right_corner, simulate_first }
    }

    /// Returns true if the position is in boundaries of memory map described by 
    /// the `self.bottom_right_corner`
    fn contains(&self, pos: UPosition) -> bool {
        pos.row <= self.bottom_right_corner.row && pos.col <= self.bottom_right_corner.col
    }

    /// Creates a hash set with corrupted byte positions
    fn initialise_corrupted_byte_map(&self, input: &FallingBytes) -> Result<HashSet<UPosition>, String> {
        let FallingBytes(bytes) = input;
        let mut corrupted = HashSet::new();
        for i in 0..self.simulate_first {
            let BytePosition { x, y } = bytes[i];
            let position = UPosition { row: y, col: x };
            if !self.contains(position) {
                return Err(error::byte_out_of_bounds(i, bytes[i], self.bottom_right_corner));
            }
            corrupted.insert(position);
        }
        Ok(corrupted)
    }

    fn next_states(&self, state: State, corrupted: &HashSet<UPosition>) -> Vec<State> {
        let State { pos, steps } = state;
        let mut next = vec![];
        for movement in movement::unit::all_partial() {
            let next_position = movement.apply(pos);
            if next_position.is_none() { continue; }

            let next_position = next_position.unwrap();
            if self.contains(next_position) && !corrupted.contains(&next_position) {
                next.push(State { pos: next_position, steps: steps + 1 });
            }
        }
        next
    }

    /// Runs breadth first search. Returns mapping of the form: Position -> Number of steps to reach it.
    /// If the position is not included, it is not reachable
    pub fn run_bfs(&self, start_position: UPosition, input: &FallingBytes) -> Result<HashMap<UPosition, usize>, String> {

        // check that memory map contains start position
        if !self.contains(start_position) {
            return Err(error::start_out_of_bounds(start_position, self.bottom_right_corner));
        }

        // try create map of corrupted memory bytes
        let corrupted_map = match self.initialise_corrupted_byte_map(input) {
            Ok(corrupted_map) => corrupted_map,
            Err(e) => return Err(e),
        };
        
        // initialise start position
        let mut steps_mapping = HashMap::new();
        let mut queue = VecDeque::new();
        if !corrupted_map.contains(&start_position) {
            let starting_state = State { pos: start_position, steps: 0 };
            queue.push_front(starting_state);
            steps_mapping.insert(start_position, starting_state.steps);
        }

        // do BFS
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            for next_state in self.next_states(current, &corrupted_map) {
                if !steps_mapping.contains_key(&next_state.pos) {
                    steps_mapping.insert(next_state.pos, next_state.steps);
                    queue.push_back(next_state);
                }
            }
        }
        
        Ok(steps_mapping)
    }
}

impl Solve<FallingBytes> for MemorySpacePathFinder {
    fn solve(&self, input: FallingBytes) -> Result<Answer, String> {
        let step_mapping = match self.run_bfs(UPosition::new((0, 0)), &input) {
            Ok(step_mapping) => step_mapping,
            Err(e) => return Err(e),
        };

        step_mapping.get(&self.bottom_right_corner)
            .map(|&steps|DisplayableAnswer::new(steps))
            .ok_or_else(||error::unreachable(self.bottom_right_corner))
    }
}