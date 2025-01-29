use std::{collections::HashSet, vec};

use crate::{answer::Report, helper::display::vector_display};

use super::models::XY;

#[derive(Debug)]
pub struct SnapshotsAnswer {
    snapshots: Vec<Vec<XY>>,
    area: XY,
}

impl SnapshotsAnswer {
    pub fn new(snapshots: Vec<Vec<XY>>, width: u16, height: u16) -> SnapshotsAnswer {
        SnapshotsAnswer { snapshots, area: XY { x: width as i32, y: height as i32 } }
    }

    fn snapshot_to_string(&self, snapshot: &Vec<XY>) -> String {
        let mut hashed_positions = HashSet::new();
        snapshot.iter().for_each(|&XY { x, y }|{ hashed_positions.insert((x, y)); });

        let mut snapshot_print = vec![];
        for y in 0..self.area.y {
            let mut row = vec![];
            for x in 0..self.area.x {
                row.push(if hashed_positions.contains(&(x, y)) { '#' } else { ' ' });
            }
            snapshot_print.push(vector_display(&row, ""));
        }
        vector_display(&snapshot_print, "\n")
    }
}

impl Report for SnapshotsAnswer {
    fn report(&self) -> String {
        let mut builder = vec![];
        for i in 0..self.snapshots.len() {
            builder.push(format!("Iteration {}:", i));
            builder.push(format!("{}", self.snapshot_to_string(&self.snapshots[i])));
        }
        vector_display(&builder, "\n")
    }
}