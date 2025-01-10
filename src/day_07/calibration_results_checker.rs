use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::{equation::{Equation, EquationList}, operation::Operation};

pub struct CalibrationResultsChecker {
    operations: Vec<Box<dyn Operation>>,
}

mod error {
    const PREFIX: &str = "[Solver D-07 P1]";

    pub fn overflow_error() -> String {
        format!("{}", PREFIX)
    }
}

impl CalibrationResultsChecker {
    pub fn new(operations: Vec<Box<dyn Operation>>) -> CalibrationResultsChecker {
        CalibrationResultsChecker { operations }
    }

    fn check(&self, equation: &Equation) -> bool {
        if equation.right_values.is_empty() { return equation.left_value == 0; }

        let mut inspection_queue = vec![(equation.left_value, equation.right_values.len() - 1)];
        while !inspection_queue.is_empty() {
            let (goal, idx) = inspection_queue.pop().unwrap();
            let current_value = equation.right_values[idx];
            if idx == 0 && goal == current_value {
                return true;
            } else if idx > 0 {
                for op in self.operations.iter() {
                    if let Some(left_component) = op.get_left_component(goal, current_value) {
                        inspection_queue.push((left_component, idx - 1));
                    }
                }
            }
        }
        
        false
    }

    pub fn safe_sum(acc: u64, value: u64) -> Result<u64, String> {
        acc.checked_add(value).ok_or_else(||error::overflow_error())
    }
}

impl Solve<EquationList> for CalibrationResultsChecker {
    fn solve(&self, input: EquationList) -> Result<Answer, String> {
        let EquationList(equations) = input;
        equations.into_iter()
            .filter(|equation|self.check(equation))
            .map(|equation|equation.left_value)
            .try_fold(0, Self::safe_sum)
            .map(DisplayableAnswer::new)
    }
}