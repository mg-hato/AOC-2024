#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_15::make_pipeline, testing::test_whole_flow};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_15/test/big_example.txt", // Example given on AOC24
        "src/day_15/test/small_example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_big_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(10_092));
    }

    #[test]
    pub fn test_whole_flow_part_1_small_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(2028));

    }
}