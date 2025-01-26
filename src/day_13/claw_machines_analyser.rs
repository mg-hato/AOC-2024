use crate::{answer::{Answer, DisplayableAnswer}, helper::result::{self, collect}, solver::Solve};

use super::{model::{ClawMachine, ClawMachines, Position}, single_solution_solver::SingleSolutionSolver};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Solver D-13]";

    pub fn token_sum_overflow(acc: u64, presses: (u64, u64)) -> String {
        let (a_presses, b_presses) = presses;
        vector_display(&vec![
            format!("{} an overflow occurred during summing up of tokens.", PREFIX),
            format!("Calculating token sum overflowed."),
            format!("Last step: accumulator={}, A={}, B={}.", acc, a_presses, b_presses),
        ], " ")
    }

    pub fn tweak_overflow(prize_coordinate: u64, tweak: u64, coordinate_name: &str) -> String {
        vector_display(&vec![
            format!("{} an overflow occurred during claw machine tweaking.", PREFIX),
            format!("Prize's {} coordinate: {}.", coordinate_name, prize_coordinate),
            format!("{}-tweak: {}", coordinate_name, tweak),
        ], " ")
    }
}

/// Claw machine analyser analyses claw machines inputs and works out how many tokens are needed.
pub struct ClawMachineAnalyser {
    prize_x_tweak: u64,
    prize_y_tweak: u64,
}

impl ClawMachineAnalyser {
    /// Claw machine analyser that tweaks each claw machine's prize position
    pub fn new_with_tweak(prize_x_tweak: u64, prize_y_tweak: u64) -> ClawMachineAnalyser {
        ClawMachineAnalyser { prize_x_tweak, prize_y_tweak }
    }

    /// Tweaks all claw machines prize positions
    fn tweak(&self, machine: ClawMachine) -> Result<ClawMachine, String> {
        let Position { x: px, y: py } = machine.prize;
        let tweaked_x = px.checked_add(self.prize_x_tweak)
            .ok_or_else(||error::tweak_overflow(px, self.prize_x_tweak, "X"));

        let tweaked_y = py.checked_add(self.prize_y_tweak)
            .ok_or_else(||error::tweak_overflow(py, self.prize_y_tweak, "Y"));
        
        result::zip(tweaked_x, tweaked_y, |tx, ty|ClawMachine { prize: Position { x: tx, y: ty }, ..machine })
    }

    fn accumulate_token_sum(token_acc: u64, presses: (u64, u64)) -> Result<u64, String> {
        let (a_presses, b_presses) = presses;
        a_presses.checked_mul(3)
            .and_then(|a_tokens|a_tokens.checked_add(b_presses))
            .and_then(|tokens|token_acc.checked_add(tokens))
            .ok_or_else(||error::token_sum_overflow(token_acc, presses))
    }
}

impl Solve<ClawMachines> for ClawMachineAnalyser {
    fn solve(&self, input: ClawMachines) -> Result<Answer, String> {
        let ClawMachines(claw_machines) = input;
        
        let tweaked_claw_machines = collect(claw_machines.into_iter().map(|machine|self.tweak(machine)).collect());
        if let Err(tweak_err) = tweaked_claw_machines { return Err(tweak_err); }

        let solutions = collect(tweaked_claw_machines.unwrap().into_iter().map(SingleSolutionSolver::solve).collect());
        if let Err(e) = solutions { return Err(e); }

        solutions.unwrap().into_iter().filter_map(|x|x)
            .try_fold(0, Self::accumulate_token_sum)
            .map(DisplayableAnswer::new)
    }
}