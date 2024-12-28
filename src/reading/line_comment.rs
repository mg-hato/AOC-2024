
/// An option for a comment that is quite common in coding.
/// A line comment - wherever the pattern is noticed, everything till the end of the line is ignored.
#[allow(dead_code)]
pub enum LineComment {
    None,
    Pattern(String),
}

pub mod reading_only {

    use crate::reader::{Line, VecLine};

    use super::*;

    fn trim_comment(line: Line, pattern: &String) -> Line {
        let trimmed = line.textf().split(pattern).map(String::from).collect::<Vec<_>>()[0].clone();
        Line::new(trimmed, line.numberf())
    }

    pub fn apply(line_comment: &LineComment, vec_line: VecLine) -> VecLine {
        let lines = vec_line.lines;
        let processed_lines = match line_comment {
            LineComment::Pattern(pattern) => lines.into_iter()
                .map(|line| trim_comment(line, pattern))
                .collect(),

            _ => lines
        };
        VecLine::new(processed_lines)
    }
}
