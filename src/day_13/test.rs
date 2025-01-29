#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_13::{make_pipeline, model::{ClawMachine, Position}, single_solution_solver::SingleSolutionSolver}, testing::test_whole_flow};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_13/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(480));
    }

    #[test]
    pub fn test_whole_flow_part_2_example() {
        let tokens_needed: u64 = 875_318_608_908; // Calculated using Google sheets. See example.txt for more steps with numbers
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(tokens_needed));

    }

    #[test]
    pub fn test_equation_solver() {
        for (machine, solution) in [
            // Example from AOC website
            (ClawMachine { 
                button_a: Position { x: 94, y: 34 },
                button_b: Position { x: 22, y: 67 },
                prize:    Position { x: 8400, y: 5400 },
            }, Some((80, 40))),
            
            // Press button A 21 times
            (ClawMachine { 
                button_a: Position { x: 10, y: 20 },
                button_b: Position { x: 37, y: 37 },
                prize:    Position { x: 210, y: 420 },
            }, Some((21, 0))),
            
            // Similar: press button B 21 times
            (ClawMachine { 
                button_a: Position { x: 37, y: 37 },
                button_b: Position { x: 10, y: 20 },
                prize:    Position { x: 210, y: 420 },
            }, Some((0, 21))),

            // Any press would drive past the prize
            (ClawMachine { 
                button_a: Position { x: 37, y: 37 },
                button_b: Position { x: 10, y: 20 },
                prize:    Position { x: 5, y: 10 },
            }, None),

            // Press each A and B exactly once
            (ClawMachine { 
                button_a: Position { x: 10, y: 15 },
                button_b: Position { x: 15, y: 10 },
                prize:    Position { x: 25, y: 25 },
            }, Some((1, 1))),
        ] {
            assert_eq!(SingleSolutionSolver::solve(machine), Ok(solution));
        }
    }

    
    #[test]
    pub fn test_equation_solver_assumptions_check() {
        for machine in [
            // Same gradient of button A and B
            ClawMachine { 
                button_a: Position { x: 10, y: 20 },
                button_b: Position { x: 15, y: 30 },
                prize:    Position { x: 15_000, y: 30_000 },
            },

            // We have zero movement on Y-axis for button A
            // I don't want to handle this edge case if I don't have to
            ClawMachine { 
                button_a: Position { x: 10, y: 0 },
                button_b: Position { x: 15, y: 30 },
                prize:    Position { x: 15_000, y: 30_000 },
            },
        ] {
            assert!(SingleSolutionSolver::solve(machine).is_err());
        }
    }
}