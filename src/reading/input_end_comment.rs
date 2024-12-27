
/// An option for a comment that denotes the end of input.
/// Meaning, if it matches a given pattern, anything in the file
/// that comes after this type of comment will be dropped out.
/// Similar to the idea of a block comment, but always extended to the end of file.
#[allow(dead_code)]
pub enum InputEndComment {
    None,
    Pattern(String),
}

pub mod reading_only {
    use crate::reader::{Line, VecLine};

    use super::*;

    fn split_line(line: Line, pattern: &String) -> Vec<Option<Line>> {
        let line_number = line.number();
        let mut splits : Vec<_> = line.text().split(pattern)
            .map(|s| Some(Line::new(String::from(s), line_number))) 
            .collect();

        // We mark the end of input by putting Option::None where the cut-off is
        if splits.len() > 1 {
            splits.insert(1, None);
        }
        splits
    }

    pub fn apply(input_end_comment: &InputEndComment, lines: VecLine) -> VecLine
    {
        match input_end_comment {
            InputEndComment::Pattern(pattern) => lines.into_iter()
                .flat_map(|line| split_line(line, pattern))
                .map_while(|opt| opt)
                .collect(),

            _ => lines
        }
    }
}