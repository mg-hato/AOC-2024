use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NumberPairList {
    pub list: Vec<NumberPair>
}

impl NumberPairList {
    pub fn new(list: Vec<NumberPair>) -> NumberPairList {
        NumberPairList { list }
    }
}

impl Display for NumberPairList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", vector_display(&self.list, ","))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NumberPair(pub u32, pub u32);

impl NumberPair {
    pub fn fst(&self) -> u32 { self.0 }
    pub fn snd(&self) -> u32 { self.1 }
}

impl Display for NumberPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}