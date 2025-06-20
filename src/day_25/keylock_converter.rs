use crate::day_25::{key_lock::{Key, Lock}, model::{KeyLockSchema, KeyLockSchematics}};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[D-25 Key-Lock converter]";

    pub fn dimensions_mismatch(index: usize, expected: (usize, usize), actual: (usize, usize)) -> String {
        vector_display(&vec![
            format!("{} dimensions of schemas not aligned.", PREFIX),
            format!("The first schema has dimension {:?},", expected),
            format!("but schema at index {} has dimension {:?}.", index,  actual)
        ], " ")
    }

    pub fn schema_neither_key_nor_lock(index: usize) -> String {
        format!("{} schema at index {} is matching neither lock nor key", PREFIX, index)
    }
}

pub struct KeyLockConverter;

impl KeyLockConverter {
    pub fn verify(schematics: KeyLockSchematics) -> Result<KeyLockSchematics, String> 
    {
        let KeyLockSchematics(schemas) = schematics;
        let mut first_dim = None;
        for (index, KeyLockSchema(schema_table)) in schemas.iter().enumerate() {
            if first_dim.is_none() {
                first_dim = Some(schema_table.dim());
            } else if first_dim != Some(schema_table.dim()) {
                return Err(error::dimensions_mismatch(index, first_dim.unwrap(), schema_table.dim()));
            }
        }

        Ok(KeyLockSchematics(schemas))
    }

    pub fn transform(schematics: KeyLockSchematics) -> Result<(Vec<Lock>, Vec<Key>), String> {
        Self::verify(schematics).and_then(|schematics|{
            let mut keys = vec![];
            let mut locks = vec![];
            let KeyLockSchematics(schemas) = schematics;
            for (schema_idx, schema) in schemas.into_iter().enumerate() {
                match (Key::new(&schema), Lock::new(&schema)) {
                    (Ok(key), _) => keys.push(key),
                    (_, Ok(lock)) => locks.push(lock),
                    _ => return Err(error::schema_neither_key_nor_lock(schema_idx)),
                };
            }
            Ok((locks, keys))
        })
    }
}