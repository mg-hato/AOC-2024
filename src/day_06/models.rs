use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum LaboratoryMapField {
    Block,
    Free,
    Guard,
}

impl Display for LaboratoryMapField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LaboratoryMapField::Block => '#',
            LaboratoryMapField::Free => '.',
            LaboratoryMapField::Guard => '^',
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaboratoryMap {
    pub map: Vec<Vec<LaboratoryMapField>>
}

impl LaboratoryMap {
    pub fn new(map: Vec<Vec<LaboratoryMapField>>) -> LaboratoryMap {
        LaboratoryMap { map }
    } 
}

impl Display for LaboratoryMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", vector_display(&self.map.iter().map(|row|vector_display(row, "")).collect(), "\n"))
    }
}