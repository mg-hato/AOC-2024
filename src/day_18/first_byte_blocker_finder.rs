use crate::{answer::{Answer, DisplayableAnswer}, day_18::memory_space_path_finder::MemorySpacePathFinder, helper::position::UPosition, solver::Solve};

use super::model::FallingBytes;


pub struct FirstByteBlockerFinder {
    bottom_right_corner: UPosition,
}

mod error {
    const PREFIX: &str = "[Solver D18 P2]";

    pub fn no_blocking_byte() -> String {
        format!("{} if all bytes fall bottom right corner is still reachable", PREFIX)
    }
}

impl FirstByteBlockerFinder {
    pub fn new(bottom_right_corner: UPosition) -> FirstByteBlockerFinder {
        FirstByteBlockerFinder { bottom_right_corner }
    }
}

impl Solve<FallingBytes> for FirstByteBlockerFinder {
    fn solve(&self, input: FallingBytes) -> Result<Answer, String> {
        let FallingBytes(bytes) = &input;
        let start = UPosition::new((0, 0));
        let mut left = 0;
        let mut right = bytes.len() + 1;

        // let N = length of bytes array
        // for all L in [0..left]: if first L bytes fall goal is reachable
        // for all R in [right..N]: if first R bytes fall goal will not be reachable
        // further: 0 <= left <= N and 0 < right <= N + 1
        while left + 1 < right {
            let mid = (left + right) / 2;
            let solver = MemorySpacePathFinder::new(self.bottom_right_corner, mid);
            let steps_mapping = match solver.run_bfs(start, &input) {
                Ok(mapping) => mapping,
                Err(e) => return Err(e),
            };

            if steps_mapping.contains_key(&self.bottom_right_corner) {
                left = mid;
            } else {
                right = mid;
            }
        }

        bytes.get(right - 1)
            .ok_or_else(error::no_blocking_byte)
            .map(|&blocking_byte|DisplayableAnswer::new(blocking_byte))
    }
}
