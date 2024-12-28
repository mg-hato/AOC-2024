use crate::{reader::VecLine, verifier::Verify};


mod error {
    const PREFIX: &str = "[Verifier D-04]";

    pub fn width_error(line_num: usize, first_line_width: usize, width: usize) -> String {
        let additional_info =
            format!("First line width is {}. Line #{} width is {}", first_line_width, line_num, width);
        
        format!("{} line #{} has a different width from the first line. {}", PREFIX, line_num, additional_info)
    }
}

pub struct WordSearchVerifier;

impl WordSearchVerifier {
    pub fn new() -> WordSearchVerifier { WordSearchVerifier }
}

impl Verify<VecLine> for WordSearchVerifier {
    fn verify(&self, input: VecLine) -> Result<VecLine, String> {
        if input.lines.len() > 0 {
            let first_line_width = input.lines[0].text.len();
            for line in input.lines.iter() {
                if line.text.len() != first_line_width {
                    return Err(error::width_error(line.number, first_line_width, line.text.len()));
                }
            }
        }
        Ok(input)
    }
}