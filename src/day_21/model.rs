use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Codes(pub Vec<String>);


impl Display for Codes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Codes(codes) = self;
        write!(f, "[{}]", vector_display(codes, ","))
    }
}
