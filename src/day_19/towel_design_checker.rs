use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::model::TowelPatternsAndDesigns;


pub struct TowelDesignChecker<I> where I: Interpret {
    suffix_visit_count_interpreter: I 
}

mod error {
    const PREFIX: &str = "[Solver D-19]";

    pub fn overflow(lhs: u64, rhs: u64, purpose: &str) -> String {
        format!("{} overflow occured doing addition of {} and {} for {}", PREFIX, lhs, rhs, purpose)
    }
}

struct PatternSet {
    patterns: HashSet<String>,
    max_len: Option<usize>,
}

impl PatternSet {
    pub fn new(patterns: HashSet<String>, max_len: Option<usize>) -> PatternSet {
        PatternSet { patterns, max_len }
    }

    pub fn contains(&self, pattern: &String) -> bool {
        self.patterns.contains(pattern)
    }

    pub fn max_length(&self) -> usize { self.max_len.unwrap_or(0) }
}

struct Suffix(String);

impl Ord for Suffix {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Suffix(lhs) = self;
        let Suffix(rhs) = other;
        lhs.len().cmp(&rhs.len())
    }
}

impl Eq for Suffix {}

impl PartialEq for Suffix {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Suffix {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <I: Interpret> TowelDesignChecker<I> {
    pub fn new(interpreter: I) -> TowelDesignChecker<I> {
        TowelDesignChecker { suffix_visit_count_interpreter: interpreter }
    }

    fn make_pattern_set(input: &TowelPatternsAndDesigns) -> PatternSet {
        let mut set = HashSet::new();
        let mut max_len = None;
        for i in 0..input.patterns.len() {
            let pattern = &input.patterns[i];
            set.insert(pattern.clone());
            if max_len.is_none_or(|val|val < pattern.len()) {
                max_len = Some(pattern.len())
            }

        }
        PatternSet::new(set, max_len)
    }

    fn analyse_design(design: &String, pattern_set: &PatternSet) -> Result<HashMap<String, u64>, String> {
        let mut suffix_visit_count = HashMap::new();
        suffix_visit_count.insert(design.clone(), 1u64);

        let mut suffix_queue = BinaryHeap::new();
        suffix_queue.push(Suffix(design.clone()));
        
        while !suffix_queue.is_empty() {
            let Suffix(current_suffix) = suffix_queue.pop().unwrap();
            let current_count = *suffix_visit_count.get(&current_suffix).unwrap();
            let max_cutoff =  (pattern_set.max_length() + 1).min(current_suffix.len() + 1);

            for cutoff in 1..max_cutoff {
                let prefix = current_suffix.get(..cutoff).unwrap().to_string();
                if !pattern_set.contains(&prefix) { continue; }

                let next_suffix = current_suffix.strip_prefix(&prefix).unwrap().to_string();
                if let Some(&previous_count) = suffix_visit_count.get(&next_suffix) {
                    let updated_count = previous_count.checked_add(current_count);
                    if updated_count.is_none() {
                        let purpose = format!("design analysis of '{}'", design);
                        return Err(error::overflow(previous_count, current_count, &purpose));
                    }
                    suffix_visit_count.insert(next_suffix, updated_count.unwrap());
                    continue;
                }

                suffix_queue.push(Suffix(next_suffix.clone()));
                suffix_visit_count.insert(next_suffix, current_count);
            }
        }

        Ok(suffix_visit_count)
    }
}

impl <I: Interpret> Solve<TowelPatternsAndDesigns> for TowelDesignChecker<I> {
    fn solve(&self, input: TowelPatternsAndDesigns) -> Result<Answer, String> {
        let pattern_set = Self::make_pattern_set(&input);
        let mut result_sum: u64 = 0;
        for design in input.designs {
            let suffix_visit_count = match Self::analyse_design(&design, &pattern_set) {
                Err(e) => return Err(e),
                Ok(analysis_result) => analysis_result,
            };

            let design_result = self.suffix_visit_count_interpreter.interpret(suffix_visit_count);
            
            match result_sum.checked_add(design_result) {
                Some(value) => result_sum = value,
                None => return Err(error::overflow(result_sum, design_result, "resulting sum calculation")),
            };
        }
        Ok(DisplayableAnswer::new(result_sum))
    }
}

pub trait Interpret {
    fn interpret(&self, suffix_visit_count: HashMap<String, u64>) -> u64;
}

pub struct PossibilityInterpreter;

impl Interpret for PossibilityInterpreter {
    fn interpret(&self, suffix_visit_count: HashMap<String, u64>) -> u64 {
        if suffix_visit_count.contains_key(&format!("")) { 1 } else { 0 }
    }
}

pub struct DifferentWaysInterpreter;

impl Interpret for DifferentWaysInterpreter {
    fn interpret(&self, suffix_visit_count: HashMap<String, u64>) -> u64 {
        *suffix_visit_count.get(&format!("")).unwrap_or(&0)
    }
}