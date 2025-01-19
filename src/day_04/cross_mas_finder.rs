
use crate::helper::{boundary::apply, movement::{self}, option::pair_merge, position::UPosition, table::Table};

use super::find::Find;


pub struct CrossMasFinder {
    wordsearch: Table<char>,
}

impl CrossMasFinder {
    pub fn new(wordsearch: Table<char>) -> CrossMasFinder {
        CrossMasFinder { wordsearch }
    }

    fn is_position_ms(&self, first: UPosition, second: UPosition) -> bool {
        match (self.wordsearch.get_pos(first.pos()), self.wordsearch.get_pos(second.pos())) {
            (Some(&fst), Some(&snd)) => Self::is_ms(fst, snd),
            _ => false
        }
    }

    // are the two characters forming 'M' and 'S'
    fn is_ms(fst: char, snd: char) -> bool {
        (fst == 'M' || fst == 'S')
        && (snd == 'M' || snd == 'S')
        && fst != snd
    }

    fn is_cross_mas_at(&self, pos: UPosition) -> bool {
        self.wordsearch.get_pos(pos.pos()).is_some_and(|&c|c == 'A') // center letter is 'A'
        && [movement::unit::UP_LEFT, movement::unit::UP_RIGHT].iter().all(|&movement|{
            let first = apply(self.wordsearch.boundary(), movement, pos);
            let second = apply(self.wordsearch.boundary(), movement.inverse(), pos);
            pair_merge(first, second).is_some_and(|(fst, snd)|self.is_position_ms(fst, snd))
        })
    }
}

impl Find for CrossMasFinder {
    fn find_all(&self) -> usize {
        self.wordsearch.iter()
            .filter(|&(pos, _)|self.is_cross_mas_at(pos))
            .count()
    }
}