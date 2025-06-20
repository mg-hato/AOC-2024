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

impl KeyLockSchema {
    pub fn columns(&self) -> Vec<Vec<KeyLockSpace>> {
        let mut columns = vec![];
        let KeyLockSchema(table) = self;
        let (row_count, columns_count) = table.dim();
        for column_idx in 0..columns_count {
            let mut current_column = vec![];
            for row_idx in 0..row_count {
                current_column.push(*table.get(row_idx, column_idx).unwrap());
            }
            columns.push(current_column);
        }
        columns
    }
}

impl Display for KeyLockSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let KeyLockSchema(schema) = self;
        write!(f, "(({}))", schema)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum KeyLockSpace {
    Block,

    Space,
}

impl Display for KeyLockSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            KeyLockSpace::Block => '#',
            KeyLockSpace::Space => '.',
        })
    }
}