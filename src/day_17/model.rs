use std::fmt::Display;

use crate::helper::display::vector_display;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProgramInformation {
    pub register_a: u64,
    pub register_b: u64,
    pub register_c: u64,
    pub program: Vec<u64>,
}

impl Display for ProgramInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", vector_display(&vec![
            format!("A : {}", self.register_a),
            format!("B : {}", self.register_b),
            format!("C : {}", self.register_c),
            format!("Program : {}", vector_display(&self.program, ",")),
        ], "; "))
    }
}