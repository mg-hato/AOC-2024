#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_18::{first_byte_blocker_finder::FirstByteBlockerFinder, make_pipeline, memory_space_path_finder::MemorySpacePathFinder, model::BytePosition}, helper::position::UPosition, solver::Solve, testing::{self}};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_18/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        let input = testing::get_parsed_result_ok(&pipeline, REL_FILEPATHS[0]);
        let solution = MemorySpacePathFinder::new(UPosition::new((6, 6)), 12).solve(input);
        assert!(solution.is_ok());
        assert_eq!(solution.unwrap().report(), DisplayableAnswer::new(22).report());
    }

    #[test]
    pub fn test_whole_flow_part_2_example() {
        let pipeline = make_pipeline(false).unwrap();
        let input = testing::get_parsed_result_ok(&pipeline, REL_FILEPATHS[0]);
        let solution = FirstByteBlockerFinder::new(UPosition::new((6, 6))).solve(input);
        assert!(solution.is_ok());
        assert_eq!(solution.unwrap().report(), DisplayableAnswer::new(BytePosition::new(6, 1)).report());
    }
}