#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_19::make_pipeline, testing::{self}};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_19/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(6));
    }

    #[test]
    pub fn test_whole_flow_part_2_example() {
        let pipeline = make_pipeline(true).unwrap();
        testing::test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(16));
    }
}