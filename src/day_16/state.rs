use crate::helper::{direction::Direction, position::UPosition};


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
/// A state is a position of the reindeer and the direction they are facing
pub struct State {
    pub position: UPosition,
    pub direction: Direction,
}

#[derive(Eq, Clone, Copy)]
/// `StateWithScore` is `State` joined with the score associated to the state.
/// Mainly used to define `Ord` to be used with `BinaryHeap`
pub struct StateWithScore {
    pub state: State,
    pub score: u64,
}

impl Ord for StateWithScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for StateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StateWithScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}