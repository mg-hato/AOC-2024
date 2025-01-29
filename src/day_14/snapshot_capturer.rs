use crate::{answer::Answer, solver::Solve};

use super::{models::{RobotList, XY}, robots_prediction_model::RobotsPredictionModel, snapshots_answer::SnapshotsAnswer};

/// Captures snapshots of robots positions at `i`th second where `0 <= i <= seconds`.
pub struct SnapshotCapturer {
    prediction_model: RobotsPredictionModel,
    seconds: u16,
}

impl SnapshotCapturer {
    pub fn new(max_seconds: u16, width: u16, height: u16) -> Result<SnapshotCapturer, String> {
        RobotsPredictionModel::new(width, height).map(|prediction_model|SnapshotCapturer {
            seconds: max_seconds, prediction_model,
        })
    }
}

impl Solve<RobotList> for SnapshotCapturer {
    fn solve(&self, input: RobotList) -> Result<Answer, String> {
        let mut i = 0;
        let mut snapshots = vec![];
        while i <= self.seconds {
            match self.prediction_model.predict(&input, i) {
                Ok(positions) => { snapshots.push(positions); },
                Err(err) => return Err(err),
            }
            i += 1;
        }
        let XY { x, y } = self.prediction_model.get_area();
        Ok(Box::new(SnapshotsAnswer::new(snapshots, x as u16, y as u16)))
    }
}