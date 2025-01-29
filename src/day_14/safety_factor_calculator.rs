use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::{models::{RobotList, XY}, robots_prediction_model::RobotsPredictionModel};

pub struct SafetyFactorCalculator {
    seconds: u16,
    prediction_model: RobotsPredictionModel,
}


mod error {
    const PREFIX: &str = "[Solver D-14 P-1]";

    pub fn product_overflow(quadrants: [u64; 4]) -> String {
        format!("{} an overflow happened while calculating safety score. Quadrants: {:?}.", PREFIX, quadrants)
    }
}

impl SafetyFactorCalculator {
    pub fn new(seconds: u16, width: u16, height: u16) -> Result<SafetyFactorCalculator, String> {
        RobotsPredictionModel::new(width, height).map(|prediction_model|SafetyFactorCalculator {
            seconds, prediction_model,
        })
    }

    fn safe_product(quadrants: [u64; 4]) -> Result<u64, String> {
        quadrants.clone().into_iter().try_fold(1u64, |acc, q|acc.checked_mul(q))
            .ok_or_else(||error::product_overflow(quadrants))
    }

    fn safety_score(&self, positions: Vec<XY>) -> Result<u64, String> {
        let mut qcounts: [u64; 4] = [0, 0, 0, 0];
        let (mid_x, mid_y) = {
            let XY { x, y } = self.prediction_model.get_area();
            (x / 2, y / 2)
        };

        for XY { x, y } in positions {
            if x == mid_x || y == mid_y { continue; }
            let x_index = if x < mid_x { 0 } else { 1 };
            let y_index = if y < mid_y { 0 } else { 1 };
            let index = y_index * 2 + x_index;
            qcounts[index] += 1;
        }

        Self::safe_product(qcounts)
    }
}

impl Solve<RobotList> for SafetyFactorCalculator {
    fn solve(&self, input: RobotList) -> Result<Answer, String> {
        self.prediction_model.predict(&input, self.seconds)
            .and_then(|positions|self.safety_score(positions))
            .map(DisplayableAnswer::new)
    }
}