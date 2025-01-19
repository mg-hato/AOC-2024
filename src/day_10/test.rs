#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_10::make_pipeline, testing::test_whole_flow};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_10/test/example.txt", // Example given on AOC24
    ];
    
    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(36));
    }
}