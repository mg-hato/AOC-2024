use crate::day_25::model::{KeyLockSchema, KeyLockSpace};

mod error {
    pub fn invalid_schema_dimensions(dim: (usize, usize), purpose: &str) -> String {
        format!("Invalid dimensions of schema {:?} for {}", dim, purpose)
    }

    pub fn column(index: usize, purpose: &str) -> String {
        format!("Column at index {} not valid for {}", index, purpose)
    }
}

fn get_column_values(schema: &KeyLockSchema, purpose: &str, top: KeyLockSpace) -> Result<Vec<usize>, String> {
        let KeyLockSchema(table) = schema;
        let (row_count, column_count) = table.dim();
        if row_count < 2 || column_count == 0 {
            return Err(error::invalid_schema_dimensions(table.dim(), purpose));
        }
        let mut column_values = vec![];
        for (column_idx, column) in schema.columns().into_iter().enumerate() {
            let top_count = column.iter().take_while(|&&field|field == top).count();
            let bottom_count = column.iter().rev().take_while(|&&field|field != top).count();
            if top_count == 0 || bottom_count == 0 || top_count + bottom_count != column.len() {
                return Err(error::column(column_idx, purpose))
            }
            let column_value = column.iter().filter(|&&field|field == KeyLockSpace::Block).count();
            column_values.push(column_value);
        }
        Ok(column_values)
}



pub struct Key {
    column_values: Vec<usize>,
}

impl Key {
    pub fn new(schema: &KeyLockSchema) -> Result<Key, String> {
        get_column_values(schema, "key", KeyLockSpace::Space)
            .map(|column_values|Key { column_values })
    }
}

pub struct Lock {
    column_values: Vec<usize>,
    height: usize,
}

impl Lock {
    pub fn new(schema: &KeyLockSchema) -> Result<Lock, String> {
        let KeyLockSchema(table) = schema;
        let (height, _) = table.dim();
        get_column_values(schema, "lock", KeyLockSpace::Block)
            .map(|column_values|Lock { column_values, height })
    }

    pub fn key_fits(&self, key: &Key) -> bool {
        if self.column_values.len() != key.column_values.len() {
            return false;
        }
        for (lock_value, key_value) in self.column_values.iter().zip(key.column_values.iter()) {
            if lock_value + key_value > self.height {
                return false;
            }
        }
        true
    }
}