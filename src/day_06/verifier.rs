use crate::verifier::Verify;

use super::models::{LaboratoryMap, LaboratoryMapField};

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[Verifier D-06]";

    pub fn not_rectangular_error(first_width: usize, other_width: usize, row_index: usize) -> String {
        vector_display(&vec![
            format!("{} map is expected to be rectangulard shaped.", PREFIX),
            format!("However, that is not the case."),
            format!("Row at index {} is of width {},", row_index, other_width),
            format!("whereas row at index 0 has width {}.", first_width),
        ], " ")
    }

    pub fn wrong_guard_count_error(count: usize) -> String {
        vector_display(&vec![
            format!("{} exactly one guard is expected on the map,", PREFIX),
            format!("but on the map there are {} guards.", count),
        ], " ")
    }
}

pub struct LaboratoryMapVerifier;

impl LaboratoryMapVerifier {
    pub fn new() -> LaboratoryMapVerifier { LaboratoryMapVerifier }

    pub fn verify_rectangular_map(input: LaboratoryMap) -> Result<LaboratoryMap, String> {
        for row in 1..input.map.len() {
            if input.map[row].len() != input.map[0].len() {
                return Err(error::not_rectangular_error(input.map[0].len(), input.map[row].len(), row))
            }
        }
        Ok(input)
    }
    
    pub fn verify_one_guard(input: LaboratoryMap) -> Result<LaboratoryMap, String> {
        match input.map.iter().flat_map(|row|row.iter())
            .filter(|&&field|field == LaboratoryMapField::Guard)
            .count() {
                1 => Ok(input),
                count => Err(error::wrong_guard_count_error(count))
            }
    }
}

impl Verify<LaboratoryMap> for LaboratoryMapVerifier {
    fn verify(&self, input: LaboratoryMap) -> Result<LaboratoryMap, String> {
        Ok(input)
            .and_then(Self::verify_rectangular_map)
            .and_then(Self::verify_one_guard)
    }
}