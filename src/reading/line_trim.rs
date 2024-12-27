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

    fn apply_trim_fn(lines: Vec<Line>, trim_fn: Box<dyn Fn(String) -> String>) -> Vec<Line> {
        lines.into_iter()
            .map(|line| Line::new(trim_fn(line.text()), line.number()))
            .collect()
    }

    pub fn apply(line_trim: &LineTrim, vec_line: VecLine) -> VecLine {
        let lines = vec_line.lines;
        let processed_lines = match line_trim {
            LineTrim::Both => apply_trim_fn(lines, Box::new(|s: String| s.trim().to_owned())),
            LineTrim::Start => apply_trim_fn(lines, Box::new(|s: String| s.trim_start().to_owned())),
            LineTrim::End => apply_trim_fn(lines, Box::new(|s: String| s.trim_end().to_owned())),
            _ => lines
        };
        VecLine::new(processed_lines)
    }
}