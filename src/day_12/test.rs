#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_12::make_pipeline, testing::test_whole_flow};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_12/test/first_example.txt", // Example given on AOC24
        "src/day_12/test/second_example.txt", // Example given on AOC24
        "src/day_12/test/big_example.txt", // Example given on AOC24
    ];
    
    #[test]
    pub fn test_whole_flow_pt1_first_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(140));
    }
    
    #[test]
    pub fn test_whole_flow_pt1_second_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(772));
    }
    
    #[test]
    pub fn test_whole_flow_pt1_big_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(1930));
    }
}