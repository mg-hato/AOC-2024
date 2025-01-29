#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_14::{make_pipeline, safety_factor_calculator::SafetyFactorCalculator}, solver::Solve, testing::get_verified_result_ok};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_14/test/example.txt", // Example given on AOC24
    ];

    

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        let input = get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
        let solver = SafetyFactorCalculator::new(100, 11, 7).unwrap();
        assert_eq!(solver.solve(input).unwrap().report(), DisplayableAnswer::new(12).report())
    }
}