use crate::{answer::{Answer, DisplayableAnswer}, helper::result::collect, solver::Solve};

use super::models::{Robot, RobotList, XY};

pub struct SafetyFactorCalculator {
    seconds: u16,
    area: XY,
}

mod error {
    use crate::{day_14::models::{Robot, XY}, helper::display::vector_display};

    const PREFIX: &str = "[Solver D-14]";

    pub fn inappropriate_area(area: XY) -> String {
        vector_display(&vec![
            format!("{} cannot create solver. Inappropriate area provided: {}.", PREFIX, area),
            format!("An appropriate area has odd non-negative width and height.")
        ], " ")
    }

    pub fn robot_position_out_of_area(robot: Robot) -> String {
        format!("{} robot's initial position is out of area defined for the solver. Robot: {}.", PREFIX, robot)
    }
}

impl SafetyFactorCalculator {
    pub fn new(seconds: u16, area: XY) -> Result<SafetyFactorCalculator, String> {
        let XY { x, y } = area;
        if x > 0 && y > 0 && x % 2 == 1 && y % 2 == 1 {
            Ok(SafetyFactorCalculator { seconds, area })
        } else {
            Err(error::inappropriate_area(area))
        }
    }

    fn check_initial_positions(&self, input: RobotList) -> Result<Vec<Robot>, String> {
        let RobotList(robots) = input;
        collect(robots.into_iter().map(|robot|{
            let XY { x, y } = robot.position;
            if 0 <= x && x < self.area.x && 0 <= y && y < self.area.y { Ok(robot) }
            else { Err(error::robot_position_out_of_area(robot)) }
        }).collect())
    }

    fn predict_position(&self, robot: Robot) -> XY {
        let Robot {
            position: XY { x: px, y: py },
            velocity: XY { x: vx, y: vy },
        } = robot;
        let time = self.seconds as i32;
        
        XY {
            x: (((px + vx * time) % self.area.x) + self.area.x) % self.area.x,
            y: (((py + vy * time) % self.area.y) + self.area.y) % self.area.y,
        }
    }

    fn safety_score(&self, positions: Vec<XY>) -> usize {
        let mut qcounts = [0, 0, 0, 0];
        let mid_x = self.area.x / 2;
        let mid_y = self.area.y / 2;

        for XY { x, y } in positions {
            if x == mid_x || y == mid_y { continue; }
            let x_index = if x < mid_x { 0 } else { 1 };
            let y_index = if y < mid_y { 0 } else { 1 };
            let index = y_index * 2 + x_index;
            qcounts[index] += 1;
        }

        qcounts.iter().product()
    }
}

impl Solve<RobotList> for SafetyFactorCalculator {
    fn solve(&self, input: RobotList) -> Result<Answer, String> {
        let checked = self.check_initial_positions(input);
        if checked.is_err() { return Err(checked.unwrap_err()); }

        let positions = checked.unwrap().into_iter()
            .map(|robot|self.predict_position(robot))
            .collect();

        Ok(DisplayableAnswer::new(self.safety_score(positions)))
    }
}