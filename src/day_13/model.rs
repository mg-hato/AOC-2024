use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position { pub x: u64, pub y: u64 }

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// Claw machine's specification represented with three `UPosition`.
/// - `button_a` is a position reached from `(0,0)` after pressing it once
/// - `button_b` same as `button_a`
/// - `prize` actual position of the prize
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ClawMachine {
    pub button_a: Position,
    pub button_b: Position,
    pub prize: Position,
}

impl ClawMachine {
    fn position_display_with_character(position: Position, c: char) -> String {
        format!("X{}{},Y{}{}", c, position.x, c, position.y)
    }

    fn button_display(button: Position) -> String {
        Self::position_display_with_character(button, '+')
    }

    fn prize_display(prize: Position) -> String {
        Self::position_display_with_character(prize, '=')
    }
}

impl Display for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ A: {}; B: {}; Prize: {} }}",
            Self::button_display(self.button_a),
            Self::button_display(self.button_b),
            Self::prize_display(self.prize))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClawMachines(pub Vec<ClawMachine>);

impl Display for ClawMachines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ClawMachines(claw_machines) = self;
        write!(f, "[{}]", vector_display(claw_machines, ", "))
    }
}
