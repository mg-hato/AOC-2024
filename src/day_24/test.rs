#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_24::{assisted_analyser::AssistedAnalyser, make_pipeline}, testing};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_24/test/small_example.txt", // Example given on AOC24
        "src/day_24/test/example.txt", // Example given on AOC24
        "src/day_24/test/my_example.txt",
        "src/day_24/test/my_wrong_example.txt",
    ];

    #[test]
    pub fn test_whole_flow_part_1_small_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new(4));
    }
    
    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[1], DisplayableAnswer::new(2_024));
    }

    #[test]
    pub fn test_scenarios_part_2_my_example() {
        let pipeline = make_pipeline(true).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[2], AssistedAnalyser::all_pass_answer());
    }

    #[test]
    pub fn test_scenarios_part_2_my_wrong_example() {
        let pipeline = make_pipeline(true).unwrap();
        let answer = testing::get_answer_ok(&pipeline, &REL_FILEPATHS[3]);
        // we expect some scenario groups to not be successful
        assert_ne!(answer.report(), AssistedAnalyser::all_pass_answer().report())
    }
}