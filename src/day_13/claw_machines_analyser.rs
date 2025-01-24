use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::model::{ClawMachine, ClawMachines};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Solver D-13]";

    pub fn overflow(acc: u64, next: u64) -> String {
        vector_display(&vec![
            format!("{} an overflow occurred during summing up of tokens.", PREFIX),
            format!("Calculating {} + {} resulted in overflow.", acc, next),
        ], " ")
    }
}

pub struct ClawMachineAnalyser;

impl ClawMachineAnalyser {
    pub fn calculate_tokens(machine: ClawMachine) -> Option<u64> {
        let ClawMachine { button_a, button_b, prize } = machine;
        let mut i = 0;
        let mut rtn = None;
        while button_a.row * i <= prize.row && button_a.col * i <= prize.col {
            let leftover_row = prize.row - button_a.row * i;
            let leftover_col = prize.col - button_a.col * i;
            i += 1;
            if leftover_row % button_b.row != 0 || leftover_col % button_b.col != 0 { continue; }
            let mul_row = leftover_row / button_b.row;
            let mul_col = leftover_col / button_b.col;
            if mul_row == mul_col {
                let current_value = (i - 1) * 3 + mul_row;
                
                rtn = rtn.filter(|&(_, _, tokens)|tokens > current_value)
                    .or(Some((i - 1, mul_row, current_value)));
            }
        }
        rtn.map(|(_, _, tokens)|tokens as u64)
    }

    fn sum_step(token_acc: u64, tokens: u64) -> Result<u64, String> {
        token_acc.checked_add(tokens).ok_or_else(||error::overflow(token_acc, tokens))
    }
}

impl Solve<ClawMachines> for ClawMachineAnalyser {
    fn solve(&self, input: ClawMachines) -> Result<Answer, String> {
        let ClawMachines(claw_machines) = input;
        claw_machines.into_iter()
            .filter_map(Self::calculate_tokens)
            .try_fold(0, Self::sum_step)
            .map(DisplayableAnswer::new)
    }
}