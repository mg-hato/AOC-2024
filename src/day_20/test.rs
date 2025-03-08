#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_20::{cheats_counter::CheatsCounter, make_pipeline}, solver::Solve, testing::{self}};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_20/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        let input = testing::get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
        let answer = CheatsCounter::new(2, 2).solve(input);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap().report(), DisplayableAnswer::new(14 + 14 + 2 + 4 + 2 + 3 + 5).report());
    }

    #[test]
    pub fn test_whole_flow_part_2_example() {
        let pipeline = make_pipeline(false).unwrap();
        let input = testing::get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
        let answer = CheatsCounter::new(20, 50).solve(input);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap().report(), DisplayableAnswer::new(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3).report());
    }
}