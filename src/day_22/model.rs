use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Numbers(pub Vec<u64>);

impl Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Numbers(numbers) = self;
        write!(f, "[{}]", vector_display(numbers, "; "))
    }
}