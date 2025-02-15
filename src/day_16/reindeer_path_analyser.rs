use std::collections::{HashMap, HashSet};

use crate::helper::{direction::Direction, position::UPosition};

use super::state::{State, StateWithScore};


pub trait ReindeerPathAnalyser {
    fn analyse(&self, scores: HashMap<State, u64>, end: UPosition) -> Result<u64, String>;
}

fn end_position_not_reachable() -> String {
    format!("[ReindeerPathAnalyser] end position is not reachable.")
}


fn scores_at(position: UPosition, scores: &HashMap<State, u64>) -> Vec<StateWithScore> {
    Direction::all()
        .into_iter()
        .map(|direction|State { direction, position })
        .flat_map(|state|scores.get(&state).map(|&score|StateWithScore { state, score }))
        .collect()
}

/// Analyses the scores and returns the smallest score required to reach end position
pub struct LowestScoreAnalyser;

impl ReindeerPathAnalyser for LowestScoreAnalyser {
    fn analyse(&self, scores: HashMap<State, u64>, end: UPosition) -> Result<u64, String> {
        // we do `.max()` here because we are returned a vector of `StateWithScore`
        // whose `Ord` relation is defined inversely on the `.score`
        scores_at(end, &scores)
            .into_iter()
            .max()
            .map(|state_with_score|state_with_score.score)
            .ok_or_else(end_position_not_reachable)
    }
}

/// Analyses the scores and returns the number of different fields that belong to an optimal path
pub struct OptimalPathFieldAnalyser;

impl OptimalPathFieldAnalyser {
    fn min_scores_at(position: UPosition, scores: &HashMap<State, u64>) -> Vec<StateWithScore> {
        let states_with_scores = scores_at(position, scores);
        let min_score = states_with_scores.clone().into_iter().max().map(|sws|sws.score);

        states_with_scores.into_iter()
            .filter(|sws|min_score.is_some_and(|score| score == sws.score))
            .collect()
    }

    fn previous_states(scores: &HashMap<State, u64>, state_score: StateWithScore) -> Vec<StateWithScore> {
        let StateWithScore { state: State { position, direction }, score } = state_score;
        let mut previous = vec![];

        // Undo rotation
        for previous_direction in [direction.rotate(), direction.rotate().rotate().rotate()] {
            if score < 1_000 { continue; }
            let previous_state = State { direction: previous_direction, position };
            if scores.get(&previous_state).is_some_and(|&act_score| act_score == score - 1_000) {
                previous.push(StateWithScore { state: previous_state, score: score - 1_000 });
            }
        }

        // Undo moving forward
        direction.rotate().rotate().movement().apply(position)
            .map(|previous_position| State { position: previous_position, direction })
            .filter(|&previous_state|scores.get(&previous_state).is_some_and(|&act_score| act_score + 1 == score))
            .map(|previous_state| StateWithScore { state: previous_state, score: score - 1 })
            .into_iter().for_each(|previous_state|{ previous.push(previous_state); });
        
        previous
    }
}

impl ReindeerPathAnalyser for OptimalPathFieldAnalyser {
    fn analyse(&self, scores: HashMap<State, u64>, end: UPosition) -> Result<u64, String> {
        let mut queue = Self::min_scores_at(end, &scores);
        if queue.len() == 0 {
            return Err(end_position_not_reachable());
        }

        let mut optimal = HashSet::new();
        queue.iter().for_each(|state_with_score|{ optimal.insert(state_with_score.state); });

        // Work out backway what states participated in the optimal paths
        while !queue.is_empty() {
            let state_with_score = queue.pop().unwrap();
            for previous_state_with_score in Self::previous_states(&scores, state_with_score) {
                if optimal.insert(previous_state_with_score.state) {
                    queue.push(previous_state_with_score);
                }
            }
        }

        // Work out only the number of positions used by the optimal paths
        let mut optimal_positions = HashSet::new();
        optimal.into_iter().for_each(|state|{ optimal_positions.insert(state.position); });
        
        Ok(optimal_positions.len() as u64)
    }
}