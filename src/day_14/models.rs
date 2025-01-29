use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

impl Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Robot {
    pub position: XY,
    pub velocity: XY,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{p:{},x:{}}}", self.position, self.velocity)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RobotList(pub Vec<Robot>);

impl Display for RobotList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let RobotList(robots) = self;
        write!(f, "[{}]", vector_display(robots, ","))
    }
}