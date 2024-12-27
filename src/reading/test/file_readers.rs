#[cfg(test)]
pub mod suite {

    use crate::{reader::{Line, Read, VecLine}, testing::resolve_filepath};

    use super::super::super::*;

    const TESTFILE_RELATIVE_PATH : &str = "src/reading/test/testfile.txt";
    
    #[test]
    fn test_simple_file_reader() {
        let filepath = resolve_filepath(TESTFILE_RELATIVE_PATH);
        let reader = SimpleFileReader::new();
        let result = reader.read(&filepath);
        assert!(result.is_ok());
        let lines = result.unwrap();
        assert_eq!(lines.len(), 4);
    }

    fn test_sanitised_file_reader_helper<R>(sfr: R, expected: VecLine) where R: Read + 'static {
        let res = sfr.read(&resolve_filepath(TESTFILE_RELATIVE_PATH));
        assert!(res.is_ok());
        let lines = res.unwrap();
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_without_sanitisation() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::None,
            settings::EmptyLineTrimming::None,
        );
        let expected = vec![
            Line::new(String::from(""),                     1),
            Line::new(String::from("   This is a test   "), 2),
            Line::new(String::from(""),                     3),
            Line::new(String::from("   "),                  4),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_empty_line_trimming_start() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::None,
            settings::EmptyLineTrimming::Start,
        );
        let expected = vec![
            Line::new(String::from("   This is a test   "), 2),
            Line::new(String::from(""),                     3),
            Line::new(String::from("   "),                  4),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_empty_line_trimming_end() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::None,
            settings::EmptyLineTrimming::End,
        );
        let expected = vec![
            Line::new(String::from(""),                     1),
            Line::new(String::from("   This is a test   "), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_empty_line_trimming_both() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::None,
            settings::EmptyLineTrimming::Both,
        );
        let expected = vec![
            Line::new(String::from("   This is a test   "), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }
    
    #[test]
    fn test_line_trim_both() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::Both,
            settings::EmptyLineTrimming::Both,
        );
        let expected = vec![
            Line::new(String::from("This is a test"), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_line_comment() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::Pattern(String::from("a")),
            settings::InputEndComment::None,
            settings::LineTrim::Both,
            settings::EmptyLineTrimming::Both,
        );
        let expected = vec![
            Line::new(String::from("This is"), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_line_comment_another() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::Pattern(String::from("is")),
            settings::InputEndComment::None,
            settings::LineTrim::Both,
            settings::EmptyLineTrimming::Both,
        );
        let expected = vec![
            Line::new(String::from("Th"), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    #[test]
    fn test_end_of_input_comment() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::Pattern(String::from("This")),
            settings::LineTrim::None,
            settings::EmptyLineTrimming::None,
        );
        let expected = vec![
            Line::new(String::from(""),    1),
            Line::new(String::from("   "), 2), // cutoff at start of 'This', three white spaces are left untouched
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }


    #[test]
    fn test_line_trim_start() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::Start,
            settings::EmptyLineTrimming::All,
        );
        let expected = vec![
            Line::new(String::from("This is a test   "), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }

    
    #[test]
    fn test_line_trim_end() {
        let sfr = SanitisedFileReader::new(
            SimpleFileReader::new(),
            settings::LineComment::None,
            settings::InputEndComment::None,
            settings::LineTrim::End,
            settings::EmptyLineTrimming::All,
        );
        let expected = vec![
            Line::new(String::from("   This is a test"), 2),
        ];
        test_sanitised_file_reader_helper(sfr, expected);
    }
}