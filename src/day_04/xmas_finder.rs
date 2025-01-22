use crate::helper::{boundary::{self, Boundary}, movement::{self, Movement}, position::UPosition, table::Table};

use super::find::Find;


pub struct XMasFinder {
    wordsearch: Table<char>,
    xmas: Vec<char>,
}

impl XMasFinder {
    pub fn new(wordsearch: Table<char>) -> XMasFinder {
        let xmas = "XMAS".chars().collect();
        XMasFinder { wordsearch, xmas }
    }

    fn find_at_with(&self, starting_position: UPosition, movement: Movement) -> bool {
        let mut i = 0;
        let mut position = self.wordsearch.boundary().bound(starting_position);
        while i < self.xmas.len() && position.is_some() {
            let pos = position.unwrap();
            
            // If out of bounds or the letter does not match next expected in "XMAS": break
            if *self.wordsearch.get_pos(pos).unwrap() != self.xmas[i] { break; }

            position = boundary::apply(self.wordsearch.boundary(), movement, pos);
            i += 1;
        }

        // If i has reached the length of "XMAS", then the word was found!
        i == self.xmas.len()
    }

    fn count_at(&self, pos: UPosition) -> usize {
        // If not starting with 'X', early quit
        let mut count = 0;
        if self.wordsearch.get_pos(pos).is_none_or(|c|*c != self.xmas[0]) {
            return 0;
        }

        for movement in movement::unit::all() {
            if self.find_at_with(pos, movement) {
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