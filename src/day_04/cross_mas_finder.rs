use crate::reader::VecLine;

use super::find::Find;


pub struct CrossMasFinder {
    wordsearch: Vec<Vec<char>>,
}

impl CrossMasFinder {
    pub fn new(input: &VecLine) -> CrossMasFinder {
        let wordsearch = input.lines.iter().map(|line|line.text.chars().collect()).collect();
        CrossMasFinder { wordsearch }
    }

    fn surroundings_check(&self, row: usize, col: usize) -> bool {
        row != 0 && col != 0 // neither are at 0-border
            && row + 1 < self.wordsearch.len() // next row exists
            && col + 1 < self.wordsearch[row - 1].len() // next column exists in previous row
            && col + 1 < self.wordsearch[row + 1].len() // next column exists in next row
    }

    // are the two characters forming 'M' and 'S'
    fn is_mas(fst: char, snd: char) -> bool {
        (fst == 'M' || fst == 'S')
        && (snd == 'M' || snd == 'S')
        && fst != snd
    }

    fn is_cross_mas_at(&self, row: usize, col: usize) -> bool {
        // If letter 'A' is not @(row, col) or if surrounding positions are unavailable/unsuitable
        if self.wordsearch[row][col] != 'A' || !self.surroundings_check(row, col) {
            return false;
        }
        // Check if other 4 letters are 'M' and 'S' and that they can all form a cross "MAS"
        Self::is_mas(self.wordsearch[row-1][col-1], self.wordsearch[row+1][col+1])
        && Self::is_mas(self.wordsearch[row-1][col+1], self.wordsearch[row+1][col-1])
    }
}

impl Find for CrossMasFinder {
    fn find_all(&self) -> usize {
        let mut counter = 0;
        for row in 0..self.wordsearch.len() {
            for col in 0..self.wordsearch[row].len() {
                if self.is_cross_mas_at(row, col) {
                    counter += 1;
                }
            }
        }
        counter
    }
}