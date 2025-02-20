#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_17::make_pipeline, helper::display::vector_display, testing::test_whole_flow};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_17/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_first_example() {
        let output = vec![4,6,3,5,6,3,5,2,1,0];
        let result = vector_display(&output, ",");
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(result));
    }
}