use std::collections::HashSet;

use crate::{answer::DisplayableAnswer, solver::Solve};

use super::models::{PageOrderingRule, RulesWithUpdates, UpdatePages};


pub struct UpdatesChecker;

mod error {
    const PREFIX: &str = "[Solver D-05 P1]";
    pub fn overflow_error() -> String {
        format!("{} an overflow error occurred when summing middle pages", PREFIX)
    }
}

impl UpdatesChecker {

    pub fn new() -> UpdatesChecker { UpdatesChecker }

    /// Transform rules into a hash-set s.t. for a rule `x|y` we put a tuple `(x,y)`
    /// in the hash-set.
    fn make_rules_hashset(input: &RulesWithUpdates) -> HashSet<(&u32, &u32)> {
        let mut hash_set = HashSet::new();
        for PageOrderingRule(fst, snd) in input.rules.iter() {
            hash_set.insert((fst, snd));
        }
        hash_set
    }

    fn is_order_correct(hrules: &HashSet<(&u32, &u32)>, update: &UpdatePages) -> bool {
        let UpdatePages(pages) = update;
        for left in 0..pages.len() {
            for right in left + 1..pages.len() {
                // for indices `left` and `right` where `left < right` we check that pages at those positions
                // do not violate the page ordering rules by checking the absence of `page[right]|page[left]` rule.
                let inversed = (&pages[right], &pages[left]);
                if hrules.contains(&inversed) {
                    return false;
                }
            }
        }
        true
    }

    /// Returns only the updates whose page order is correct or incorrect, based on the `correct_order_criteria`.
    /// If `correct_order_criteria` is `true`, returns only correctly ordered updates.
    /// Otherwise, it returns only incorrectly ordered updates.
    pub fn get_updates(input: &RulesWithUpdates, correct_order_criteria: bool) -> Vec<&UpdatePages> {
        let hrules = Self::make_rules_hashset(input);
        input.updates.iter()
            .filter(|update|Self::is_order_correct(&hrules, update) == correct_order_criteria)
            .collect()
    }

    pub fn middle(update: &UpdatePages) -> u32 {
        let UpdatePages(pages) = update;
        // len = 2k + 1, so we need `pages[k]` (pages[0..k) first k pages, pages[k+1,2k+1) last k pages)
        let k = pages.len();
        pages[k / 2]
    }

    fn safe_add(acc: u32, page: u32) -> Result<u32, String> {
        acc.checked_add(page)
            .ok_or_else(error::overflow_error)
    }
}

impl Solve<RulesWithUpdates> for UpdatesChecker {
    fn solve(&self, input: RulesWithUpdates) -> Result<crate::answer::Answer, String> {
        Self::get_updates(&input, true)
            .into_iter()
            .map(Self::middle)
            .try_fold(0u32, Self::safe_add)
            .map(DisplayableAnswer::new)
    }
}