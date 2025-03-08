use std::collections::{HashMap, HashSet, VecDeque};

use crate::{answer::{Answer, DisplayableAnswer}, helper::{movement::{unit, Movement}, position::UPosition, table::Table}, solver::Solve};

use super::model::Field;


pub struct CheatsCounter {
    cheat_time: usize,
    cheat_cutoff: u64,
}

mod error {
    use crate::helper::{display::vector_display, position::UPosition};

    const PREFIX: &str = "[Solver D-20]";

    pub fn not_exactly_one(c: char, name: &str, count: usize) -> String {
        vector_display(&vec![
            format!("{} The input map has {} {}s (character: {}).", PREFIX, count, name, c),
            format!("Exactly one is expected."),
        ], " ")
    }

    pub fn unreachable(start: UPosition, end: UPosition) -> String {
        format!("{} end position {} not reachable from start {}", PREFIX, end, start)
    }
}

impl CheatsCounter {
    pub fn new(cheat_time: usize, cheat_cutoff: u64,) -> CheatsCounter {
        CheatsCounter { cheat_cutoff, cheat_time }
    }

    /// Process the input table / map. Returns in a tuple, in order:
    /// 1. A hashset of all walkable positions (i.e. non-wall positions)
    /// 2. Initial position.
    /// 3. An end position that needs to be reached.
    fn process_input_map(input: Table<Field>) -> Result<(HashSet<UPosition>, UPosition, UPosition), String> {
        let mut racetrack = HashSet::new();
        let mut starts = vec![];
        let mut ends = vec![];

        for (position, &field) in input.iter() {
            if field != Field::Wall { racetrack.insert(position); }
            if field == Field::Start { starts.push(position); }
            else if field == Field::End { ends.push(position); }
        }

        if starts.len() != 1 {
            return Err(error::not_exactly_one('S', "starting position", starts.len()));
        }
        
        if ends.len() != 1 {
            return Err(error::not_exactly_one('E', "end position", ends.len()));
        }

        Ok((racetrack, starts[0], ends[0]))
    }

    /// Returns a mapping: position => distance from goal
    fn do_bfs(racetrack: &HashSet<UPosition>, goal: UPosition) -> HashMap<UPosition, u64> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        if racetrack.contains(&goal) {
            queue.push_front(goal);
            distances.insert(goal, 0);
        }

        while !queue.is_empty() {
            let current_pos = queue.pop_front().unwrap();
            let distance = *distances.get(&current_pos).unwrap();
            for movement in unit::all_partial() {
                let next = movement.apply(current_pos);
                if next.is_some() && racetrack.contains(&next.unwrap()) && !distances.contains_key(&next.unwrap()) {
                    distances.insert(next.unwrap(), distance + 1);
                    queue.push_back(next.unwrap());
                }
            }
        }

        distances
    }

    fn cheat_movements(&self) -> Vec<Movement> {
        let origin = UPosition::new((self.cheat_time + 1, self.cheat_time + 1));
        let mut i = 0;
        let mut reached = HashSet::new();
        reached.insert(origin);
        let unit_movs = unit::all_partial();
        while i < self.cheat_time {
            i += 1;

            let newly_reached = reached.iter()
                .flat_map(|&pos|unit_movs.iter().map(move |m|m.apply(pos)))
                .collect::<Vec<_>>();

            for new_pos in newly_reached {
                if let Some(pos) = new_pos {
                    reached.insert(pos);
                }
            }

        }
        reached.into_iter().map(|pos|Movement::infer(origin, pos)).collect()
    }

    fn count_cheats(&self, distances: &HashMap<UPosition, u64>) -> u64 {
        let movements = self.cheat_movements();
        let mut cheat_count = 0;
        for (&pos, &distance) in distances.iter() {
            for movement in movements.iter() {
                let next = movement.apply(pos);
                if next.is_none() { continue; }

                let next = next.unwrap();
                let next_distance = distances.get(&next);
                if next_distance.is_none() { continue; }

                let next_distance = *next_distance.unwrap();
                let cheat_distance = (movement.col.get_absolute_change() + movement.row.get_absolute_change()) as u64;
                if next_distance + cheat_distance >= distance { continue; }

                let saved_time = distance - next_distance - cheat_distance;
                if saved_time >= self.cheat_cutoff {
                    cheat_count += 1;
                }
            }
        }
        cheat_count
    }

}

impl Solve<Table<Field>> for CheatsCounter {
    fn solve(&self, input: Table<Field>) -> Result<Answer, String> {
        let (racetrack, start, end) = match Self::process_input_map(input) {
            Ok((r, s, e)) => (r,s,e),
            Err(e) => return Err(e),
        };
        
        let distances = Self::do_bfs(&racetrack, end);
        if !distances.contains_key(&start) {
            return Err(error::unreachable(start, end));
        }

        Ok(DisplayableAnswer::new(self.count_cheats(&distances)))
    }
}