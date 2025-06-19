use std::fmt::Display;

use crate::helper::{display::vector_display, table::Table};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyLockSchematics(pub Vec<KeyLockSchema>); 

impl Display for KeyLockSchematics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let KeyLockSchematics(schemas) = self;
        write!(f, "[{}]", vector_display(schemas, ";"))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct KeyLockSchema(pub Table<KeyLockSpace>);

impl Display for KeyLockSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let KeyLockSchema(schema) = self;
        write!(f, "(({}))", schema)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum KeyLockSpace {
    Busy,

    Free,
}

impl Display for KeyLockSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            KeyLockSpace::Busy => '#',
            KeyLockSpace::Free => '.',
        })
    }
}