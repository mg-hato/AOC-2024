#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_21::{make_pipeline, model::Codes}, solver::Solve, testing::{self}};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_21/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new(126_384));
    }

    #[test]
    pub fn test_individual_codes_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        let Codes(codes) = testing::get_parsed_result_ok(&pipeline, &REL_FILEPATHS[0]);
        let result = pipeline.solve(Codes(vec![codes[0].clone()]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().report(), DisplayableAnswer::new(68 * 29).report())
    }
}