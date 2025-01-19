use std::collections::{HashMap, HashSet};

use crate::{answer::{Answer, DisplayableAnswer}, helper::{boundary::apply, movement, position::UPosition, table::Table}, solver::Solve};

pub struct TrailheadCounter;

impl TrailheadCounter {
    fn start(map: &Table<usize>, start_height: usize) -> HashMap<UPosition, HashSet<UPosition>> {
        let mut rtn = HashMap::new();
        for position in map.iter().filter(|(_, &height)|height == start_height).map(|(pos,_)|pos) {
            let mut positions = HashSet::new();
            positions.insert(position);
            rtn.insert(position, positions);
        }
        rtn
    }

    fn next(map: &Table<usize>, current: HashMap<UPosition, HashSet<UPosition>>, next_height: usize)
        -> HashMap<UPosition, HashSet<UPosition>> {
        let mut rtn = HashMap::new();
        for (position, origins) in current {
            for movement in movement::unit::all_partial() {
                // if next position out of map or not next appropriate height: skip
                let next_position = apply(map.boundary(), movement, position);
                if next_position.is_none() || *map.get_pos(next_position.unwrap().pos()).unwrap() != next_height {
                    continue;
                }

                let next_position = next_position.unwrap();
                if !rtn.contains_key(&next_position) { rtn.insert(next_position, HashSet::new()); }
                let next_origins = rtn.get_mut(&next_position).unwrap();
                origins.iter().for_each(|&origin|{ next_origins.insert(origin); });
            }
        }
        rtn        
    }
}

impl Solve<Table<usize>> for TrailheadCounter {
    fn solve(&self, input: Table<usize>) -> Result<Answer, String> {
        let mut height = 9;
        let mut state = Self::start(&input, height);
        while height > 0 {
            height = height - 1;
            state = Self::next(&input, state, height)
        }
        Ok(DisplayableAnswer::new(state.into_iter().map(|(_, origins)|origins.len()).sum::<usize>()))
    }
}