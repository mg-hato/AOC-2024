#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_01::{make_pipeline, models::{NumberPair, NumberPairList}}, testing};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_01/test/example.txt", // Example given on AOC24
        "src/day_01/test/another_example.txt",
        "src/day_01/test/overflow_example.txt",
        "src/day_01/test/overflowed_input.txt",
    ];

    
    #[test]
    pub fn test_parser() {        
        testing::test_parsing(
            &make_pipeline(false).unwrap(),
            REL_FILEPATHS[0],
            NumberPairList::new(vec![
                NumberPair(3,4),
                NumberPair(4,3),
                NumberPair(2,5),
                NumberPair(1,3),
                NumberPair(3,9),
                NumberPair(3,3),
            ]));
    }

    
    #[test]
    pub fn test_parser_overflow_number() {
        let parsing_err = testing::get_parsing_error(&make_pipeline(false).unwrap(), REL_FILEPATHS[3]);
        assert!(parsing_err.contains("4294967296")) // should report this number as failed to parse in u32
    }

    #[test]
    pub fn test_whole_flow_pt1_example() {
        testing::test_whole_flow(
            &make_pipeline(false).unwrap(),
            REL_FILEPATHS[0],
            DisplayableAnswer::new(11u32));
    }


    #[test]
    pub fn test_whole_flow_pt2_example() {
        testing::test_whole_flow(
            &make_pipeline(true).unwrap(),
            REL_FILEPATHS[0],
            DisplayableAnswer::new(31u32));
    }

    #[test]
    pub fn test_whole_flow_pt1_another_example() {
        testing::test_whole_flow(
            &make_pipeline(false).unwrap(),
            REL_FILEPATHS[1],
            DisplayableAnswer::new(8u32));
    }

    #[test]
    pub fn test_whole_flow_pt2_another_example() {
        testing::test_whole_flow(
            &make_pipeline(true).unwrap(),
            REL_FILEPATHS[1],
            DisplayableAnswer::new(10u32));
    }

    #[test]
    pub fn test_overflow_handling() {
        let overflow_err = testing::get_answer_error(&make_pipeline(false).unwrap(), REL_FILEPATHS[2]);
        assert!(overflow_err.contains("overflow"));
    }
}