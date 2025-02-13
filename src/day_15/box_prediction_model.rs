use std::{collections::{HashMap, HashSet}, vec};

use crate::{answer::{Answer, DisplayableAnswer}, day_15::models::RobotMoves, helper::{direction::Direction, position::UPosition, table::Table}, solver::Solve};

use super::{map_state::MapState, models::{Field, MapAndMoves}};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Solver D-15 P-1]";

    pub fn box_prediction_model_creation(scale: usize) -> String {
        vector_display(&vec![
            format!("{} Could not BoxPredictionModel with scale {}.", PREFIX, scale),
            format!("Scale must be greater than 0."),
        ], " ")
    }

    pub fn bad_input(robot_count: usize) -> String {
        vector_display(&vec![
            format!("{} Could not create initial state.", PREFIX),
            format!("Exactly one robot is required on the input map,"),
            format!("however, there are {} robots", robot_count),
        ], " ")
    }
}

pub struct BoxPredictionModel {
    scale: usize,
}

impl BoxPredictionModel {
    pub fn new(scale: usize) -> Result<BoxPredictionModel, String> {
        if scale == 0 {
            Err(error::box_prediction_model_creation(scale))
        } else {
            Ok(BoxPredictionModel { scale: scale.max(1) })
        }
    }

    /// Applies the scale and expands positions to the right.
    /// `position` is the leftmost position of an object with scale `self.scale`.
    /// Returns all the positions occupied by the scaled object.
    fn apply_scale(&self, position: UPosition) -> Vec<UPosition> {
        let mut expanded_positions = vec![];
        for i in 0..self.scale {
            expanded_positions.push(UPosition { col: position.col + i, ..position });
        }
        expanded_positions
    }

    /// `position` is an object's position in the unscaled map.
    /// Returns an equivalent leftmost position in the scaled up map
    fn get_leftmost_scaled_up_position(&self, position: UPosition) -> UPosition {
        UPosition { col: position.col * self.scale, ..position }
    }

    fn initial_state(&self, map: Table<Field>) -> Result<MapState, String> {
        let mut walkable_map = HashMap::new();
        let mut robot_positions = vec![];
        let mut crates = vec![];

        // Process the input map
        for (pos, &field) in map.iter() {
            // if it is a wall, nothing to do
            if field == Field::Wall { continue; }

            // if it is non-wall, box/robot can be on these positions, so we wish to add it
            // to the map of "walkable" positions
            let expanded_positions = self.apply_scale(self.get_leftmost_scaled_up_position(pos));

            // initially, some crates can be on these positions, in which case we associate them with crate id
            let value = if field == Field::Crate { Some(crates.len()) } else { None };
            
            // if it is a crate or robot here, register it in respective vector
            if field == Field::Crate {
                crates.push(expanded_positions[0]);
            } else if field == Field::Robot {
                robot_positions.push(expanded_positions[0]);
            }
            
            // populate the walkable map
            for epos in expanded_positions {
                walkable_map.insert(epos, value);
            }
        }

        // exactly one robot expected
        if robot_positions.len() != 1 {
            return Err(error::bad_input(robot_positions.len()));
        }
        
        // successfully create initial state
        Ok(MapState{
            map: walkable_map, crates,
            robot: robot_positions[0],
        })
    }

    fn try_move(
        position: UPosition, direction: Direction, map: &HashMap<UPosition, Option<usize>>
    ) -> Option<UPosition> {
        direction.movement().apply(position).filter(|moved_position|map.contains_key(moved_position))
    }

    fn get_crates_affected(&self, state: &MapState, direction: Direction) -> HashSet<usize> {
        let mut crates_affected = HashSet::new();
        let mut already_queued = HashSet::new();
        already_queued.insert(state.robot);
        let mut queue = vec![state.robot];

        while !queue.is_empty() {
            let current_pos = queue.pop().unwrap();

            // If we apply directional movement to current position and we hit wall / are out of bounds, stop processing
            let next_position = Self::try_move(current_pos, direction, &state.map);
            if next_position.is_none() { continue; }

            // If the next position is not occupied by crate, stop processing
            let crate_index = *state.map.get(&next_position.unwrap()).unwrap();
            if crate_index.is_none() { continue; }
            
            
            crates_affected.insert(crate_index.unwrap());
            
            // Otherwise, another crate is moved, which can move further crates: queue those positions for inspection
            for crate_position in self.apply_scale(state.crates[crate_index.unwrap()]) {
                if already_queued.insert(crate_position) {
                    queue.push(crate_position); // queue the position for inspection iff not queued before
                }
            }
        }
        crates_affected
    }

    fn next_state(&self, state: MapState, direction: Direction) -> MapState {
        let affected_crates = self.get_crates_affected(&state, direction);
        
        let all_crates_can_be_moved = affected_crates.iter()
            .flat_map(|&crate_index|self.apply_scale(state.crates[crate_index]))
            .all(|crate_position|Self::try_move(crate_position, direction, &state.map).is_some());

        let robot_can_be_moved = Self::try_move(state.robot, direction, &state.map).is_some();
        
        // if the robot cannot be moved or some crate cannot be moved, state does not update
        if !all_crates_can_be_moved || !robot_can_be_moved { return state; }

        // At this point change in state is expected, so we make it mutable
        let mut state = state;

        // "remove" all affected crates
        for pos in affected_crates.iter().flat_map(|&crate_index|self.apply_scale(state.crates[crate_index])) {
            state.map.insert(pos, None);
        }

        // "add" all affected crates after their unit directional move
        for crate_id in affected_crates {
            for crate_position in self.apply_scale(state.crates[crate_id]) {
                let next_position = Self::try_move(crate_position, direction, &state.map).unwrap();
                state.map.insert(next_position, Some(crate_id));
            }
            let next_crate_pos = Self::try_move(state.crates[crate_id], direction, &state.map);
            state.crates[crate_id] = next_crate_pos.unwrap();
        }

        // move the robot
        state.robot = Self::try_move(state.robot, direction, &state.map).unwrap();

        // state is fully updated
        state
    }

    fn calculate_gps_sum(state: MapState) -> usize {
        state.crates.into_iter().map(|pos|pos.row * 100 + pos.col).sum()
    }
}

impl Solve<MapAndMoves> for BoxPredictionModel {
    fn solve(&self, input: MapAndMoves) -> Result<Answer, String> {
        let MapAndMoves(map, moves) = input;
        match self.initial_state(map) {
            Ok(initial_state) => {
                let end_state = moves.into_iter()
                    .flat_map(|RobotMoves(moves)|moves)
                    .fold(initial_state, |state, dir|self.next_state(state, dir));

                Ok(DisplayableAnswer::new(Self::calculate_gps_sum(end_state)))
            }
            Err(e) => Err(e),
        }
    }
}