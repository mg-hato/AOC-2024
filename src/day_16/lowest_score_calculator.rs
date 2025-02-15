use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{answer::{Answer, DisplayableAnswer}, helper::{direction::Direction, position::UPosition, table::Table}, solver::Solve};

use super::model::Field;



pub struct LowestScoreCalculator;

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Solver D-16]";

    pub fn not_exactly_one(c: char, name: &str, count: usize) -> String {
        vector_display(&vec![
            format!("{} The input map has {} {}s (character: {}).", PREFIX, count, name, c),
            format!("Exactly one is expected."),
        ], " ")
    }
}

impl LowestScoreCalculator {
    fn make_initial_state(input: Table<Field>) -> Result<(HashSet<UPosition>, StateScore, UPosition), String> {
        let mut walkable = HashSet::new();
        let mut starts = vec![];
        let mut ends = vec![];

        for (position, &field) in input.iter() {
            if field != Field::Wall { walkable.insert(position); }
            if field == Field::Start { starts.push(position); }
            else if field == Field::End { ends.push(position); }
        }

        if starts.len() != 1 {
            return Err(error::not_exactly_one('S', "starting position", starts.len()));
        }
        
        if ends.len() != 1 {
            return Err(error::not_exactly_one('E', "end position", ends.len()));
        }

        let state_score = StateScore {
            state: State { position: starts[0], direction: Direction::Right },
            score: 0,
        };
        Ok((walkable, state_score, ends[0]))
    }

    fn next_states(map: &HashSet<UPosition>, state_score: StateScore) -> Vec<StateScore> {
        let StateScore { state: State { position, direction }, score } = state_score;
        let mut next = vec![];

        // Add state changes related to rotation
        for next_direction in [direction.rotate(), direction.rotate().rotate().rotate()] {
            next.push(StateScore {
                state: State { direction: next_direction, position },
                score: score + 1_000,
            });
        }

        // Add next state related to moving forward
        if let Some(next_position) = direction.movement().apply(position).filter(|p|map.contains(p)) {
            next.push(StateScore {
                state: State { direction, position: next_position },
                score: score + 1,
            });
        }
        
        next
    }
}

impl Solve<Table<Field>> for LowestScoreCalculator {
    fn solve(&self, input: Table<Field>) -> Result<Answer, String> {
        let initial_configuration = Self::make_initial_state(input);
        if let Err(e) = initial_configuration {
            return Err(e);
        }

        let (map, initial_state, end) = initial_configuration.unwrap();

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(initial_state); 
        let mut explored = HashMap::new();
        explored.insert(initial_state.state, initial_state.score);

        while !priority_queue.is_empty() {
            let state_score = priority_queue.pop().unwrap();
            for next in Self::next_states(&map, state_score) {
                if explored.contains_key(&next.state) { continue; }
                priority_queue.push(next);
                explored.insert(next.state, next.score);
            }
        }

        Direction::all().into_iter()
            .map(|direction|State { position: end, direction })
            .flat_map(|end_state|explored.get(&end_state).into_iter())
            .map(|&num|num).min()
            .ok_or_else(||format!("End position cannot be reached"))
            .map(DisplayableAnswer::new)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    position: UPosition,
    direction: Direction,
}

#[derive(Eq, Clone, Copy)]
struct StateScore {
    state: State,
    score: u64,
}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StateScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}