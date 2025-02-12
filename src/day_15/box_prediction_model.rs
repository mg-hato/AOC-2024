use std::collections::HashMap;

use crate::{answer::{Answer, DisplayableAnswer}, day_15::models::RobotMoves, helper::{boundary::Boundary, direction::Direction, position::UPosition, table::{Table, TableBound}}, solver::Solve};

use super::models::{Field, MapAndMoves};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Solver D-15 P-1]";

    pub fn bad_input(robot_count: usize) -> String {
        vector_display(&vec![
            format!("{} Could not create initial state.", PREFIX),
            format!("Exactly one robot is required on the input map,"),
            format!("however, there are {} robots", robot_count),
        ], " ")
    }
}

pub struct BoxPredictionModel;

impl BoxPredictionModel {
    fn initial_state(map: Table<Field>) -> Result<MapState, String> {
        let mut hashed_map = HashMap::new();
        let mut robot_positions = vec![];

        for (pos, &field) in map.iter() {
            hashed_map.insert(pos, field);
            if field == Field::Robot { robot_positions.push(pos); }
        }

        if robot_positions.len() != 1 {
            return Err(error::bad_input(robot_positions.len()));
        }

        Ok(MapState{
            map: hashed_map,
            robot_position: robot_positions[0],
            boundary: map.boundary(),
        })
    }

    fn next_state(state: MapState, direction: Direction) -> MapState {
        let mut map = state.map;

        // Next robot's position; only if it is non-wall
        let next_robot_position = state.boundary.apply(direction.movement(), state.robot_position)
            .filter(|p|map[p] != Field::Wall);

        // Crate destination is a position behind the line of crates; only if it is non-wall
        let first_crate = next_robot_position.filter(|p|map[p] == Field::Crate);
        let mut crate_destination = first_crate;
        while crate_destination.is_some_and(|p|map[&p] == Field::Crate) {
            crate_destination = state.boundary.apply(direction.movement(), crate_destination.unwrap())
        }

        // We can move things if one of the following:
        // 1. There are no crates in front of the robot and next robot's position is available (empty)
        // 2. There are crates in front of the robot and the crates can be pushed
        let robot_position = if (first_crate.is_some() && crate_destination.is_some_and(|p|map[&p] == Field::Empty))
        || (first_crate.is_none() && next_robot_position.is_some()) {

            // Move robot
            map.insert(next_robot_position.unwrap(),Field::Robot);
            map.insert(state.robot_position, Field::Empty);
            if first_crate.is_some() {
                map.insert(crate_destination.unwrap(), Field::Crate);
            }
            next_robot_position.unwrap()
        } else {
            state.robot_position
        };
        MapState { map, robot_position, boundary: state.boundary }
    }

    fn calculate_gps_sum(state: MapState) -> usize {
        state.map.into_iter().map(|(pos, field)|{
            if field == Field::Crate {
                pos.row * 100 + pos.col
            } else { 0 }
        }).sum()
    }
}

impl Solve<MapAndMoves> for BoxPredictionModel {
    fn solve(&self, input: MapAndMoves) -> Result<Answer, String> {
        let MapAndMoves(map, moves) = input;
        match Self::initial_state(map) {
            Ok(initial_state) => {
                let end_state = moves.into_iter()
                    .flat_map(|RobotMoves(moves)|moves)
                    .fold(initial_state, Self::next_state);

                Ok(DisplayableAnswer::new(Self::calculate_gps_sum(end_state)))
            }
            Err(e) => Err(e),
        }
    }
}

struct MapState {
    pub map: HashMap<UPosition, Field>,
    pub robot_position: UPosition,
    pub boundary: TableBound,
}