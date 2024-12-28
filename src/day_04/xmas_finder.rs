use crate::reader::VecLine;

use super::find::Find;


pub struct XMasFinder {
    wordsearch: Vec<Vec<char>>,
    xmas: Vec<char>,
}

impl XMasFinder {
    pub fn new(input: &VecLine) -> XMasFinder {
        let wordsearch = input.lines.iter().map(|line|line.text.chars().collect()).collect();
        let xmas = "XMAS".chars().collect();
        XMasFinder { wordsearch, xmas }
    }

    fn find_at_with<TF>(&self, row: usize, col: usize, transform: &TF) -> bool
    where TF: Fn(usize, usize) -> Option<(usize, usize)> {
        let mut i = 0;
        let mut position = Some((row, col));
        while i < self.xmas.len() && position.is_some() {
            let (r, c) = position.unwrap();
            
            // If out of bounds or the letter does not match next expected in "XMAS": break
            if r >= self.wordsearch.len() || c >= self.wordsearch[r].len() || self.wordsearch[r][c] != self.xmas[i] {
                break;
            }

            position = transform(r, c);
            i += 1;
        }

        // If i has reached the length of "XMAS", then the word was found!
        i == self.xmas.len()
    }

    fn find_at(&self, row: usize, col: usize) -> usize {
        // If not starting with 'X', early quit
        let mut count = 0;
        if self.wordsearch[row][col] != self.xmas[0] {
            return 0;
        }

        for transform in [
            // Row increasing:
            |r,c|Some((r + 1, c + 1)), // column increases
            |r,c|Some((r + 1, c)), // column stays the same
            |r,c|if c == 0 { None } else { Some((r + 1, c - 1)) }, // column decreases

            // Row static
            |r,c|Some((r, c + 1)), // column increases
            |r,c|if c == 0 { None } else { Some((r, c - 1)) }, // column decreases

            // Row decreases
            |r,c|if r == 0 { None } else { Some((r - 1, c + 1)) }, // column increases
            |r,c|if r == 0 { None } else { Some((r - 1, c)) }, // column stays the same
            |r,c|if r == 0 || c == 0 { None } else { Some((r - 1, c - 1)) }, // column decreases
        ] {
            if self.find_at_with(row, col, &transform) {
                count += 1;
            }
        }
        count
    }
}

impl Find for XMasFinder {
    fn find_all(&self) -> usize {
        let mut counter = 0;
        for row in 0..self.wordsearch.len() {
            for col in 0..self.wordsearch[row].len() {
                counter += self.find_at(row, col);
            }
        }
        counter
    }
}