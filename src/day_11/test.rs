#[cfg(test)]
pub mod suite {
    // use crate::{answer::DisplayableAnswer, day_10::make_pipeclsqline, testing::test_whole_flow};

    use crate::{answer::DisplayableAnswer, day_11::{make_pipeline, stone_prediction_model::StonePredictionModel}, solver::Solve, testing::{get_verified_result_ok, test_whole_flow}};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_11/test/example.txt", // Example given on AOC24
    ];
    
    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(55_312));
    }

    #[test]
    pub fn test_with_various_number_of_blinks() {
        let pipeline = make_pipeline(false).unwrap();
        let input = get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
        for (blinks, expected_stones) in [
            (0, 2),
            (1, 3),
            (2, 4),
            (3, 5),
            (4, 9),
            (5, 13),
            (6, 22),
        ] {
            let solver = StonePredictionModel::new(blinks);
            let result = solver.solve(input.clone());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().report(), DisplayableAnswer::new(expected_stones).report());
        }
    }
}