#[cfg(test)]
pub mod suite {
    use std::collections::HashSet;

    use crate::{answer::DisplayableAnswer, day_15::make_pipeline, testing::test_whole_flow};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_15/test/big_example.txt", // Example given on AOC24
        "src/day_15/test/small_example.txt", // Example given on AOC24
        "src/day_15/test/third_example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_part_1_big_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(10_092));
    }

    #[test]
    pub fn test_whole_flow_part_1_small_example() {
        let pipeline = make_pipeline(false).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[1], DisplayableAnswer::new(2028));

    }

    #[test]
    pub fn test_whole_flow_part_2_big_example() {
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[0], DisplayableAnswer::new(9021));
    }

    #[test]
    pub fn test_whole_flow_part_2_third_example() {
        // answer calculation in the comment section of the file
        let pipeline = make_pipeline(true).unwrap();
        test_whole_flow(&pipeline, REL_FILEPATHS[2], DisplayableAnswer::new(618));
    }

    #[test]
    pub fn test_unwapr() {
        let mut values = HashSet::new();
        let mut vec_values = vec![];
        let value = Some(3);

        values.insert(value.unwrap());
        vec_values.push(value.unwrap());
        assert_eq!(values.len(), 1);
        assert_eq!(vec_values.len(), 1);
    }
}