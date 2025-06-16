#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_23::make_pipeline, testing};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_23/test/example.txt", // Example given on AOC24
        // "src/day_22/test/example_part_2.txt", // Example given on AOC24 for part 2
    ];

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new(7));
    }
    
    #[test]
    pub fn test_whole_flow_part_2_example() {
        let pipeline = make_pipeline(true).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new("co,de,ka,ta"));
    }

    // #[test]
    // pub fn test_whole_flow_part_2_example() {
    //     let pipeline = make_pipeline(true).unwrap();
    //     testing::test_whole_flow(&pipeline, &REL_FILEPATHS[1], DisplayableAnswer::new(23));
    // }
}