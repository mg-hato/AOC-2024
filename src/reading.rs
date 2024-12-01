mod simple_file_reader;
mod sanitised_file_reader;
mod empty_line_trimming;
mod input_end_comment;
mod line_comment;
mod line;
mod line_trim;
mod test;

pub use self::simple_file_reader::SimpleFileReader;
pub use self::sanitised_file_reader::SanitisedFileReader;

#[allow(unused_imports)]
pub mod settings {
    pub use super::input_end_comment::InputEndComment;
    pub use super::line_comment::LineComment;
    pub use super::line_trim::LineTrim;
    pub use super::empty_line_trimming::EmptyLineTrimming;
}