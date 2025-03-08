use crate::helper::movement::{self, Delta, Movement};


/// OrderedMovement represents a movement where it is defined
/// which out of the two components needs to take place first.
/// The components are:
/// - Row movement
/// - Column movement
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum OrderedMovement {
    /// Row component first
    RowCol(Delta, Delta),

    /// Column component first
    ColRow(Delta, Delta),
}

impl OrderedMovement {
    pub fn create_from(movement: Movement) -> Vec<OrderedMovement> {
        let Movement { row, col } = movement;
        vec![OrderedMovement::RowCol(row, col), OrderedMovement::ColRow(col, row)]
    }

    pub fn get_absolute_change(&self) -> usize {
        match self {
            OrderedMovement::RowCol(row, col) => row.get_absolute_change() + col.get_absolute_change(),
            OrderedMovement::ColRow(col, row) => row.get_absolute_change() + col.get_absolute_change(),
        }
    }

    /// Transform ordered movement into sequence of directional buttons
    pub fn into_buttons(&self) -> Vec<char> {
        // determine the buttons & click counts
        let (
            first_click_count,
            first_button,
            second_click_count,
            second_button
        ) = match *self {
            OrderedMovement::RowCol(row, col) => (
                row.get_absolute_change(), 
                Self::map_delta(row, 'v', '^'),
                col.get_absolute_change(),
                Self::map_delta(col, '>', '<'),
            ),
            OrderedMovement::ColRow(col, row) => (
                col.get_absolute_change(),
                Self::map_delta(col, '>', '<'),
                row.get_absolute_change(), 
                Self::map_delta(row, 'v', '^'),
            ),
        };

        let mut buttons = vec![];
        (0..first_click_count).for_each(|_|buttons.push(first_button));
        (0..second_click_count).for_each(|_|buttons.push(second_button));
        buttons
    }

    fn map_delta(delta: Delta, on_inc: char, on_dec: char) -> char {
        match delta {
            Delta::Inc(_) => on_inc,
            Delta::Dec(_) => on_dec,
        }
    }

    /// Transform ordered movement into sequence of unit movements
    pub fn into_unit_movements(&self) -> Vec<Movement> {
        self.into_buttons().into_iter().map(|button|match button {
            '>' => movement::unit::RIGHT,
            '<' => movement::unit::LEFT,
            '^' => movement::unit::UP,
            _ => movement::unit::DOWN,
        }).collect()
    }
}
