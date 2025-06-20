#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_25::make_pipeline, testing};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_25/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline().unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new(3));
    }
}