#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_16::make_pipeline, testing::test_whole_flow};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_16/test/first_example.txt", // Example given on AOC24
        "src/day_16/test/second_example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_first_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(7_036));
    }

    #[test]
    pub fn test_whole_flow_part_1_second_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(11_048));

    }

    #[test]
    pub fn test_whole_flow_part_2_first_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(45));
    }

    #[test]
    pub fn test_whole_flow_part_2_second_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(64));

    }
}