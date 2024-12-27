#[allow(dead_code)]
pub enum LineTrim {
    None,
    Start,
    End,
    Both
}

pub mod reading_only {

    use crate::reader::{Line, VecLine};

    use super::*;

    fn apply_trim_fn(lines: VecLine, trim_fn: Box<dyn Fn(String) -> String>) -> VecLine {
        lines.into_iter()
            .map(|line| Line::new(trim_fn(line.text()), line.number()))
            .collect()
    }

    pub fn apply(line_trim: &LineTrim, lines: VecLine) -> VecLine {
        match line_trim {
            LineTrim::Both => apply_trim_fn(lines, Box::new(|s: String| s.trim().to_owned())),
            LineTrim::Start => apply_trim_fn(lines, Box::new(|s: String| s.trim_start().to_owned())),
            LineTrim::End => apply_trim_fn(lines, Box::new(|s: String| s.trim_end().to_owned())),
            _ => lines
        }
    }
}