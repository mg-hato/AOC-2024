use crate::helper::{option::pair_merge, table::Table};

use super::{find::Find, position_utilities::*};


pub struct XMasFinder {
    wordsearch: Table<char>,
    xmas: Vec<char>,
}

impl XMasFinder {
    pub fn new(wordsearch: Table<char>) -> XMasFinder {
        let xmas = "XMAS".chars().collect();
        XMasFinder { wordsearch, xmas }
    }

    
    fn find_at_with<TF>(&self, starting_position: (usize, usize), transform: &TF) -> bool
    where TF: Fn((usize, usize)) -> Option<(usize, usize)> {
        let mut i = 0;
        let mut position = Some(starting_position);
        while i < self.xmas.len() && position.is_some() {
            let pos = position.unwrap();
            
            // If out of bounds or the letter does not match next expected in "XMAS": break
            if self.wordsearch.get_pos(pos).is_none_or(|&c|c != self.xmas[i]) {
                break;
            }

            position = transform(pos);
            i += 1;
        }

        // If i has reached the length of "XMAS", then the word was found!
        i == self.xmas.len()
    }

    fn count_at(&self, pos: (usize, usize)) -> usize {
        // If not starting with 'X', early quit
        let mut count = 0;
        if self.wordsearch.get_pos(pos).is_none_or(|c|*c != self.xmas[0]) {
            return 0;
        }

        for transform in [
            // // Row increasing:
            |(r,c)|pair_merge(inc(r), inc(c)), // column increases
            |(r,c)|pair_merge(inc(r), same(c)), // column stays the same
            |(r,c)|pair_merge(inc(r), dec(c)), // column decreases

            // Row static
            |(r,c)|pair_merge(same(r), inc(c)), // column increases
            |(r,c)|pair_merge(same(r), dec(c)), // column decreases

            // Row decreases
            |(r,c)|pair_merge(dec(r), inc(c)), // column increases
            |(r,c)|pair_merge(dec(r), same(c)), // column stays the same
            |(r,c)|pair_merge(dec(r), dec(c)), // column decreases
        ] {
            if self.find_at_with(pos, &transform) {
                count += 1;
            }
        }
        count
    }
}

impl Find for XMasFinder {
    fn find_all(&self) -> usize {
        let mut counter = 0;
        for (pos, _) in self.wordsearch.iter() {
            counter += self.count_at(pos);
        }
        counter
    }
}