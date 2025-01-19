use std::fmt::Display;

use crate::helper::display::vector_display;

use super::{boundary::Boundary, position::UPosition};


/// Table represents a vector of vectors, a container with rows and columns, but with added
/// guarantee on the shape of the data: all rows will have equal number of entries i.e.
/// it is a rectangular structure with `N` rows with each row having `M` columns, for some
/// numbers `N` and `M`. 
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Table<T> {
    table: Vec<Vec<T>>,
    dim: (usize, usize),
}

mod error {
    use crate::helper::display::vector_display;

    pub fn table_creation_error(first_row_column_count: usize, i: usize, row_i_column_count: usize) -> String {
        vector_display(&vec![
            format!("could not create table with the input data"),
            format!("because the first row has {} columns", first_row_column_count),
            format!("but row #{} has {} columns", i, row_i_column_count)
        ], " ")
    }
}

impl <T> Table<T> {
    pub fn new(input: Vec<Vec<T>>) -> Result<Table<T>, String> {
        let row_count = input.len();
        let col_count = input.get(0).map_or(0, |first_row|first_row.len());
        for i in 0..row_count {
            if col_count != input[i].len() {
                return Err(error::table_creation_error(col_count, i, input[i].len()));
            }
        }
        // special handle when we have a table with 0 columns, but non-zero (empty) rows
        let adjusted_row_count = if col_count == 0 { 0 } else { row_count };
        let adjusted_input = if adjusted_row_count == 0 { vec![] } else { input };
        Ok(Table { table: adjusted_input, dim: (adjusted_row_count, col_count) })
    }

    #[allow(dead_code)]
    /// Returns the dimensions of the table as `(number of rows, number of columns)`
    pub fn dim(&self) -> (usize, usize) { self.dim }

    pub fn boundary(&self) -> TableBound {
        let (row, col) = self.dim;
        TableBound { row, col }
    }
    
    pub fn get_pos(&self, pos: (usize, usize)) -> Option<&T> {
        let (row, col) = pos;
        self.get(row, col)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.table.get(row).and_then(|r|r.get(col))
    }

    pub fn iter(&self) -> TableIterator<T> {
        TableIterator { table: &self, current_position: UPosition::zero() }
    }
}

impl <T> Display for Table<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_rows = self.table.iter().map(|row|vector_display(row, " ")).collect();
        write!(f, "[{}]", vector_display(&formatted_rows, ","))
    }
}

pub struct TableIterator<'a, T> {
    table: &'a Table<T>,
    current_position: UPosition,
}

impl <'a, T> Iterator for TableIterator<'a, T> {
    type Item = (UPosition, &'a T);
    
    fn next(&mut self) -> Option<Self::Item> {
        let UPosition { row: r, col: c} = self.current_position;
        let (r_len, c_len) = self.table.dim;

        if r == r_len {
            None
        } else {
            // Move column first, if new column index has been reseted to 0, move row
            let new_c = if c + 1 == c_len { 0 } else { c + 1 };
            let new_r = if new_c == 0 { r + 1 } else { r };
            self.current_position = UPosition::new((new_r, new_c));
            Some((UPosition::new((r, c)), &self.table.table[r][c]))
        }
    }
}

#[derive(Clone, Copy)]
pub struct TableBound {
    row: usize,
    col: usize,
}

impl Boundary for TableBound {
    fn bound(&self, pos: UPosition) -> Option<UPosition> {
        let UPosition { row, col } = pos;
        if row < self.row && col < self.col { Some(pos) } else { None }
    }
}