#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_12::make_pipeline, testing::test_whole_flow};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_12/test/first_example.txt", // Example given on AOC24
        "src/day_12/test/second_example.txt", // Example given on AOC24
        "src/day_12/test/big_example.txt", // Example given on AOC24
        "src/day_12/test/e_shaped_example.txt", // Example given on AOC24 for part 2 only
        "src/day_12/test/careful_example.txt", // Example given on AOC24 for part 2 only
    ];
    
    #[test]
    pub fn test_whole_flow_pt1_first_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(140));
    }
    
    #[test]
    pub fn test_whole_flow_pt2_first_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(80));
    }
    
    #[test]
    pub fn test_whole_flow_pt1_second_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(772));
    }
        
    #[test]
    pub fn test_whole_flow_pt2_second_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(436));
    }
    
    #[test]
    pub fn test_whole_flow_pt1_big_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(1930));
    }
    
    #[test]
    pub fn test_whole_flow_pt2_big_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(1206));
    }

    #[test]
    pub fn test_whole_flow_pt2_e_shaped_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[3], DisplayableAnswer::new(236));
    }

    #[test]
    pub fn test_whole_flow_pt2_careful_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[4], DisplayableAnswer::new(368));
    }
}