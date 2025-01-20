use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Stones(pub Vec<u64>);

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Stones(stones) = self;
        write!(f, "[{}]", vector_display(stones, ","))
    }
}