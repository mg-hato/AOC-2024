use std::collections::HashSet;

use crate::{day_05::models::UpdatePages, verifier::Verify};
use super::models::RulesWithUpdates;

pub struct RulesWithUpdatesVerifier;

mod error {
    const PREFIX: &str = "[Verifier D-05]";
    pub fn duplicate_page_error(index: usize, page: u32) -> String {
        format!("{} duplicate page detected in update with index {}. Page {} repeats.", PREFIX, index, page)
    }
    
    pub fn even_number_of_pages_error(index: usize) -> String {
        format!("{} update with index {} contains even number of pages.", PREFIX, index)
    }
}

impl RulesWithUpdatesVerifier {
    pub fn new() -> RulesWithUpdatesVerifier { RulesWithUpdatesVerifier }

    fn verify_no_duplicates(input: RulesWithUpdates) -> Result<RulesWithUpdates, String> {
        for i in 0..input.updates.len() {
            let UpdatePages(pages) = &input.updates[i];
            let mut set = HashSet::new();
            for page in pages.iter() {
                if set.contains(page) {
                    return Err(error::duplicate_page_error(i, *page));
                }
                set.insert(page);
            }
        }
        Ok(input)
    }

    fn verify_updates_contain_odd_number_of_pages(input: RulesWithUpdates) -> Result<RulesWithUpdates, String> {
        for i in 0..input.updates.len() {
            let UpdatePages(pages) = &input.updates[i];
            if pages.len() % 2 == 0 {
                return Err(error::even_number_of_pages_error(i));
            }
        }
        Ok(input)
    }
}

impl Verify<RulesWithUpdates> for RulesWithUpdatesVerifier {
    fn verify(&self, input: RulesWithUpdates) -> Result<RulesWithUpdates, String> {
        Ok(input)
            .and_then(Self::verify_no_duplicates)
            .and_then(Self::verify_updates_contain_odd_number_of_pages)
    }
}