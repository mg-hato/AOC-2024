use crate::reader::{Line, Read, VecLine};
pub struct SimpleFileReader;

impl SimpleFileReader {
    pub fn new() -> SimpleFileReader { SimpleFileReader }
}

impl Read for SimpleFileReader {
    fn read(&self, input_file_path: &str) -> Result<VecLine, String> {
        match std::fs::read_to_string(input_file_path) {
            Ok(text) => {
                let lines = text.lines()
                    .enumerate()
                    .map(|(line_num, line_str)| Line::new(String::from(line_str), line_num + 1))
                    .collect();
                Ok(lines)
            }
            Err(err) => Err(format!("Error when reading the file '{}': {}", input_file_path, err))
        }
    }
}