use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Equation {
    pub left_value: u64,
    pub right_values: Vec<u64>    
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} : {})", self.left_value, vector_display(&self.right_values, ","))
    }
}

impl Equation {
    pub fn new(left_value: u64, right_values: Vec<u64>) -> Equation {
        Equation { left_value, right_values }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct EquationList(pub Vec<Equation>);

impl Display for EquationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let EquationList(equations) = self;
        write!(f, "[{}]", vector_display(equations, ","))
    }
}