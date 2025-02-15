use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{answer::{Answer, DisplayableAnswer}, helper::{direction::Direction, position::UPosition, table::Table}, solver::Solve};

use super::{model::Field, reindeer_path_analyser::ReindeerPathAnalyser, state::{State, StateWithScore}};

pub struct ReindeerMazeSolver<RPA> where RPA : ReindeerPathAnalyser {
    analyser: RPA,
}

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

impl <RPA> ReindeerMazeSolver<RPA> where RPA : ReindeerPathAnalyser {

    pub fn new(analyser: RPA) -> ReindeerMazeSolver<RPA> where RPA : ReindeerPathAnalyser {
        ReindeerMazeSolver { analyser }
    }

    /// Process the input table / map. Returns in a tuple, in order:
    /// 1. A hashset of all walkable positions that reindeer can found themselves on (i.e. non-wall positions)
    /// 2. Initial state of the reindeer with its initial score of zero.
    /// 3. An end position that needs to be reached.
    fn process_input_map(input: Table<Field>) -> Result<(HashSet<UPosition>, StateWithScore, UPosition), String> {
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

        let state_score = StateWithScore {
            state: State { position: starts[0], direction: Direction::Right },
            score: 0,
        };
        Ok((walkable, state_score, ends[0]))
    }

    /// Given the hashset of walkable positions and the current state with score returns next states wiht their scores
    /// after a single step is made by reindeer. Those are:
    /// 1. Reindeer turns 90 degrees clockwise (+1,000 points)
    /// 2. Reindeer turns 90 degrees anti-clockwise (+1,000 points)
    /// 3. Reindeer moves forward one step -- if possible (+1 point)
    /// 
    /// The third state depends on whether there is a walkable field in front of the reindeer.
    fn next_states(map: &HashSet<UPosition>, state_score: StateWithScore) -> Vec<StateWithScore> {
        let StateWithScore { state: State { position, direction }, score } = state_score;
        let mut next = vec![];

        // Add state changes related to rotation
        for next_direction in [direction.rotate(), direction.rotate().rotate().rotate()] {
            next.push(StateWithScore {
                state: State { direction: next_direction, position },
                score: score + 1_000,
            });
        }

        // Add next state related to moving forward
        if let Some(next_position) = direction.movement().apply(position).filter(|p|map.contains(p)) {
            next.push(StateWithScore {
                state: State { direction, position: next_position },
                score: score + 1,
            });
        }
        
        next
    }
}

impl <RPA> Solve<Table<Field>> for ReindeerMazeSolver<RPA> where RPA : ReindeerPathAnalyser {
    fn solve(&self, input: Table<Field>) -> Result<Answer, String> {

        // work out the initial state and whether it satisfies all the assumptions
        let initial_configuration = Self::process_input_map(input);
        if let Err(e) = initial_configuration {
            return Err(e);
        }

        let (map, initial_state, end) = initial_configuration.unwrap();

        // build a priority queue +  state to score mapping: having initial state to begin with
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(initial_state); 
        let mut scores = HashMap::new();
        scores.insert(initial_state.state, initial_state.score);

        while !priority_queue.is_empty() {
            let state_score = priority_queue.pop().unwrap();
            for next in Self::next_states(&map, state_score) {
                // if next state already is in `scores` mapping
                // it means we have visited it with optimal score already: skip
                if scores.contains_key(&next.state) { continue; }
                
                priority_queue.push(next);
                scores.insert(next.state, next.score);
            }
        }
        
        self.analyser.analyse(scores, end).map(DisplayableAnswer::new)
    }
}
