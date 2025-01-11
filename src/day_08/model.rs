use std::fmt::Display;



#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AntennaMapField {
    Antenna(char),
    Free,
}

impl Display for AntennaMapField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AntennaMapField::Antenna(a) => *a,
            AntennaMapField::Free => '.',
        })
    }
}