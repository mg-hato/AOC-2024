#[allow(dead_code)]
pub enum EmptyLineTrimming {
    Start,
    End,
    Both,
    All,
    None,
}

pub mod reading_only {

    use crate::reader::{Line, VecLine};

    use super::*;

    fn is_empty_line(line: &Line) -> bool {
        line.text().trim().len() == 0
    }

    fn trim_start(lines: Vec<Line>) -> Vec<Line> {
        lines.into_iter().skip_while(is_empty_line).collect()
    }

    fn trim_end(lines: Vec<Line>) -> Vec<Line> {
        lines.into_iter().rev()
            .skip_while(is_empty_line)
            .collect::<Vec<_>>()
            .into_iter().rev()
            .collect()
    }

    fn trim_all(lines: Vec<Line>) -> Vec<Line> {
        lines.into_iter().filter(|line| !is_empty_line(line)).collect()
    }

    pub fn apply(empty_line_trimming: &EmptyLineTrimming, vec_line: VecLine) -> VecLine {
        let lines = vec_line.lines;
        let processed_lines = match empty_line_trimming {
            EmptyLineTrimming::Start => trim_start(lines),
            EmptyLineTrimming::End => trim_end(lines),
            EmptyLineTrimming::Both => trim_end(trim_start(lines)),
            EmptyLineTrimming::All => trim_all(lines),
            _ => lines,
        };
        VecLine::new(processed_lines)
    }
}