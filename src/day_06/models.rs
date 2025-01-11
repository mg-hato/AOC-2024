use std::fmt::Display;

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