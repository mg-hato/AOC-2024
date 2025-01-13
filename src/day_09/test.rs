#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_09::make_pipeline, testing::test_whole_flow};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_09/test/example.txt", // Example given on AOC24
        "src/day_09/test/all_used_memory_edge_case.txt",
        "src/day_09/test/all_free_memory_edge_case.txt",
    ];
    
    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(1_928));
    }

    #[test]
    pub fn test_whole_flow_pt2_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(2_858));
    }

    #[test]
    pub fn all_used_memory_edge_case_pt1() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(23));
    }

    #[test]
    pub fn all_used_memory_edge_case_pt2() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(23));
    }

    #[test]
    pub fn all_free_memory_edge_case_pt1() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(0));
    }

    #[test]
    pub fn all_free_memory_edge_case_pt2() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(0));
    }
}