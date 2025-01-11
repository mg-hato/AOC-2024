use crate::helper::{option::pair_merge, table::Table};

use super::{find::Find, position_utilities::*};


pub struct CrossMasFinder {
    wordsearch: Table<char>,
}

impl CrossMasFinder {
    pub fn new(wordsearch: Table<char>) -> CrossMasFinder {
        CrossMasFinder { wordsearch }
    }

    // are the two characters forming 'M' and 'S'
    fn is_ms(fst: char, snd: char) -> bool {
        (fst == 'M' || fst == 'S')
        && (snd == 'M' || snd == 'S')
        && fst != snd
    }

    fn is_cross_mas_at(&self, pos: (usize, usize)) -> bool {
        let (r, c) = pos;
        
        self.wordsearch.get_pos(pos).is_some_and(|&c|c == 'A') // center letter is 'A'
        
        // is the first diagonal forming MAS
        && pair_merge(
            pair_merge(dec(r), dec(c)).and_then(|pos|self.wordsearch.get_pos(pos)),
            pair_merge(inc(r), inc(c)).and_then(|pos|self.wordsearch.get_pos(pos))
        ).is_some_and(|(c1, c2)|Self::is_ms(*c1, *c2)) 

        // is the second diagonal forming MAS
        &&  pair_merge(
            pair_merge(dec(r), inc(c)).and_then(|pos|self.wordsearch.get_pos(pos)),
            pair_merge(inc(r), dec(c)).and_then(|pos|self.wordsearch.get_pos(pos))
        ).is_some_and(|(c1, c2)|Self::is_ms(*c1, *c2))
    }
}

impl Find for CrossMasFinder {
    fn find_all(&self) -> usize {
        self.wordsearch.iter()
            .filter(|(pos, _)|self.is_cross_mas_at(*pos))
            .count()
    }
}