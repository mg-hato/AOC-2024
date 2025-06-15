#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_22::{make_pipeline, secret_number_transform::SecretNumberTransform}, testing};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_22/test/example.txt", // Example given on AOC24
        "src/day_22/test/example_part_2.txt", // Example given on AOC24 for part 2
    ];

    #[test]
    pub fn test_transformer() {
        let transform = SecretNumberTransform::default();
        let initial_secret = 123;
        let secret_numbers = transform.iterative_evolve_sequence(initial_secret, 10);
        assert_eq!(secret_numbers, vec![
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
            ])
    }

    #[test]
    pub fn test_whole_flow_part_1_example() {
        let pipeline = make_pipeline(false).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[0], DisplayableAnswer::new(37327623));
    }

    #[test]
    pub fn test_whole_flow_part_2_example() {
        let pipeline = make_pipeline(true).unwrap();
        testing::test_whole_flow(&pipeline, &REL_FILEPATHS[1], DisplayableAnswer::new(23));
    }
}