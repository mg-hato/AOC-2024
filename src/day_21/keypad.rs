use std::collections::{HashMap, HashSet};


use crate::{day_21::ordered_movement::OrderedMovement, helper::{movement::{Movement}, position::UPosition, table::Table}};


pub struct Keypad {
    buttons: HashMap<char, UPosition>,
    button_positions: HashSet<UPosition>,
}

mod error {
    use crate::{day_21::ordered_movement::OrderedMovement, helper::display::vector_display};

    const PREFIX: &str = "[D-21 Keypad]";
    
    pub fn button_missing(button_desc: &str, button: char) -> String {
        format!("{} {} button missing: '{}'", PREFIX, button_desc, button)
    }

    pub fn safety_check_critical_error(origin: char, target: char, movement: OrderedMovement) -> String {
        vector_display(&vec![
            format!("{} a critical error occurred during safety check of ordered movement {:?}.", PREFIX, movement),
            format!("Origin button: '{}', Target button: '{}'.", origin, target),
            format!("During the check trailing position went out of bounds."),
        ], " ")
    }

    pub fn no_ordered_movements(origin: char, target: char) -> String {
        vector_display(&vec![
            format!("{} there were no safe ordered movements", PREFIX),
            format!("from button '{}' to button '{}'.", origin, target),
        ], " ")
    }
}

impl Keypad {

    fn new(layout: Table<char>) -> Keypad {
        let buttons = Self::layout_to_mapping(layout);
        let mut button_positions = HashSet::new();
        for &pos in buttons.values() {
            button_positions.insert(pos);
        }
        
        Keypad { button_positions, buttons }
    }

    /// Get viable ordered movements to get from origin button to target button.
    pub fn get_ordered_movements(&self, origin: char, target: char) -> Result<Vec<OrderedMovement>, String> {
        
        // Check buttons
        if !self.buttons.contains_key(&origin) {
            return Err(error::button_missing("origin", origin));
        }
        if !self.buttons.contains_key(&target) {
            return Err(error::button_missing("target", target));
        }

        let mut safe_ordered_movements = vec![];
        let movement = Movement::infer(self.buttons[&origin], self.buttons[&target]);
        for ord_movement in OrderedMovement::create_from(movement) {
            let mut current_pos = self.buttons[&origin];
            let mut is_safe = true;
            for unit_movement in ord_movement.into_unit_movements() {
                current_pos = match unit_movement.apply(current_pos) {
                    Some(next_pos) => next_pos,
                    None => return Err(error::safety_check_critical_error(origin, target, ord_movement)),
                };

                if !self.button_positions.contains(&current_pos) {
                    is_safe = false;
                    break;
                }
            }
            
            if is_safe { safe_ordered_movements.push(ord_movement); }
        }
        
        if safe_ordered_movements.len() == 0 {
            Err(error::no_ordered_movements(origin, target))
        } else {
            Ok(safe_ordered_movements)
        }
    }

    fn layout_to_mapping(layout: Table<char>) -> HashMap<char, UPosition> {
        let mut mapping = HashMap::new();
        for (pos, &button) in layout.iter() {
            if button == ' ' { continue; }
            mapping.insert(button, pos);
        }
        mapping
    }
}


pub mod numerical_keypad {
    use crate::{day_21::keypad::Keypad, helper::table::Table};

    /// Create new numerical keypad
    pub fn new() -> Keypad {
        Keypad::new(Table::new(vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ]).unwrap())
    }
}

pub mod directional_keypad {
    use crate::{day_21::keypad::Keypad, helper::table::Table};

    /// Create new directional keypad
    pub fn new() -> Keypad {
        Keypad::new(Table::new(vec![
            vec![' ', '^', 'A'],
            vec!['<', 'v', '>'],
        ]).unwrap())
    }
}