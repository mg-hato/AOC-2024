use super::{guard_state::GuardState, loop_detector::LoopDetector, next_state::NextState};

/// A loop detector produced from another loop detector
/// under assumption that an obstacle has been added
pub struct AdjustedLoopDetector<'a> {
    underylying: Box::<&'a mut dyn LoopDetector>,
    obstacle: (usize, usize),
}

mod error {
    const PREFIX: &str = "[AdjustedLoopDetector]";
    pub fn guard_on_obstacle(position: (usize, usize)) -> String {
        let (r, c) = position;
        format!("{} the guard cannot ever be on the placed obstacle at ({},{})", PREFIX, r, c)
    }
}

impl AdjustedLoopDetector<'_> {
    pub fn new<D>(loop_detector: &mut D, obstacle: (usize, usize)) -> AdjustedLoopDetector
    where D: LoopDetector {
        AdjustedLoopDetector { underylying: Box::new(loop_detector),  obstacle }
    }
}

impl LoopDetector for AdjustedLoopDetector<'_> {
    fn next_state(&mut self, current_state: GuardState) -> Result<NextState, String> {
        // Check that the current state does not clash with obstacle position-wise
        if current_state.position == self.obstacle {
            return Err(error::guard_on_obstacle(self.obstacle))
        }
        
        self.underylying.next_state(current_state).map(|next_state| {
            // Does the underlying returned position cross the obstacle
            let crosses_obstacle = match next_state {
                NextState::Next(GuardState { position, .. }) => 
                    current_state.is_facing(self.obstacle)
                    && !GuardState::new(position, current_state.direction).is_facing(self.obstacle),
                NextState::Out => current_state.is_facing(self.obstacle),
            };

            // If it does, get a state right in front of the obstacle, with changed direction
            if crosses_obstacle {
                NextState::Next(GuardState::new(
                    current_state.direction.rotate().rotate().next(self.obstacle).unwrap(),
                    current_state.direction.rotate(),
                ))
            } else { next_state }
        })
    }

    fn starting_state(&self) -> GuardState {
        self.underylying.starting_state()
    }
}