use crate::helper::result::collect;

use super::models::{Robot, RobotList, XY};


pub struct RobotsPredictionModel { area: XY }

mod error {
    use crate::{day_14::models::{Robot, XY}, helper::display::vector_display};

    const PREFIX: &str = "[Robots Prediction Model D-14]";

    pub fn inappropriate_area(area: XY) -> String {
        vector_display(&vec![
            format!("{} cannot create the model. Inappropriate area provided: {}.", PREFIX, area),
            format!("An appropriate area has odd width and height.")
        ], " ")
    }

    pub fn robot_position_out_of_area(robot: Robot) -> String {
        format!("{} robot's initial position is out of area defined for the solver. Robot: {}.", PREFIX, robot)
    }
}

impl RobotsPredictionModel {
    pub fn new(width: u16, height: u16) -> Result<RobotsPredictionModel, String> {
        let area = XY { x: width as i32, y: height as i32 };
        if width % 2 == 1 && height % 2 == 1 {
            Ok(RobotsPredictionModel { area })
        } else {
            Err(error::inappropriate_area(area))
        }
    }
    
    pub fn get_area(&self) -> XY { self.area }


    fn check_initial_positions(&self, input: &RobotList) -> Result<Vec<Robot>, String> {
        let RobotList(robots) = input;
        collect(robots.iter().map(|&robot|{
            let XY { x, y } = robot.position;
            if 0 <= x && x < self.area.x && 0 <= y && y < self.area.y { Ok(robot) }
            else { Err(error::robot_position_out_of_area(robot)) }
        }).collect())
    }

    fn predict_position(&self, robot: Robot, seconds: u16) -> XY {
        let Robot {
            position: XY { x: px, y: py },
            velocity: XY { x: vx, y: vy },
        } = robot;
        let time = seconds as i32;
        
        XY {
            x: (((px + vx * time) % self.area.x) + self.area.x) % self.area.x,
            y: (((py + vy * time) % self.area.y) + self.area.y) % self.area.y,
        }
    }

    pub fn predict(&self, input: &RobotList, seconds: u16) -> Result<Vec<XY>, String> {
        self.check_initial_positions(input).map(
            |robots|robots.into_iter().map(|robot|self.predict_position(robot, seconds)).collect()
        )
    }
}