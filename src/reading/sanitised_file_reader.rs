use crate::reader::Read;
use crate::reader::VecLine;

use super::empty_line_trimming;
use super::input_end_comment;
use super::input_end_comment::*;
use super::line_comment;
use super::line_comment::LineComment;
use super::line_trim;
use super::settings::EmptyLineTrimming;
use super::settings::LineTrim;
use super::SimpleFileReader;

pub struct SanitisedFileReader {
    underlying_reader: Box<dyn Read>,
    end_comment: InputEndComment,
    line_comment: LineComment,
    trim: LineTrim,
    empty_line_trim: EmptyLineTrimming,
}

impl SanitisedFileReader {
    pub fn new<R>(
        underlying_reader: R,
        line_comment: LineComment,
        input_end_comment: InputEndComment,
        line_trim: LineTrim,
        empty_line_trimming: EmptyLineTrimming,
    ) -> SanitisedFileReader where R: Read + 'static {
        SanitisedFileReader {
            underlying_reader: Box::new(underlying_reader),
            end_comment: input_end_comment,
            line_comment,
            trim: line_trim,
            empty_line_trim: empty_line_trimming,
        }
    }

    pub fn default() -> SanitisedFileReader {
    SanitisedFileReader::new(
        SimpleFileReader::new(),
        LineComment::Pattern(format!("//")),
        InputEndComment::Pattern(format!("####")),
        LineTrim::Both,
        EmptyLineTrimming::All,
    )
    }

    fn sanitise(&self, lines: VecLine) -> VecLine {
        let end_comment_trim = |lines| input_end_comment::reading_only::apply(&self.end_comment, lines);
        let line_comment_trim = |lines| line_comment::reading_only::apply(&self.line_comment, lines);
        let line_trim_fn = |lines| line_trim::reading_only::apply(&self.trim, lines);
        let empty_line_trim_fn = |lines| empty_line_trimming::reading_only::apply(&self.empty_line_trim, lines);
        empty_line_trim_fn(line_trim_fn(line_comment_trim(end_comment_trim(lines))))
    }
}

impl Read for SanitisedFileReader {
    fn read(&self, input_file_path: &str) -> Result<VecLine, String> {
        match self.underlying_reader.read(input_file_path) {
            Ok(result) => Ok(self.sanitise(result)),
            rtn => rtn
        }
    }
}