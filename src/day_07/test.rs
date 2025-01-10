#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_07::{make_pipeline, operation::{Addition, Concatenation, Multiplication, Operation}}, executer::Execute};

    const REL_FILEPATHS: &[&str] = &[
        "src/day_07/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(3_749).report());
    }

    #[test]
    pub fn test_whole_flow_pt2_example() {
        let pipeline = make_pipeline(true).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(11_387).report());
    }

    #[test]
    pub fn test_concatenation() {
        let op = |res, right|Concatenation.get_left_component(res, right);
        assert_eq!(op(1_100, 100), Some(1));
        assert_eq!(op(1_100, 10), None);
        assert_eq!(op(3_3433, 33), Some(334));
        assert_eq!(op(1_100, 0), Some(110));
        assert_eq!(op(134, 134), Some(0));
        assert_eq!(op(1_340, 134), None);
        assert_eq!(op(1_340, 1_034), None);
        assert_eq!(op(1_340, 13_400), None);
    }

    #[test]
    pub fn test_addition() {
        let op = |res, right|Addition.get_left_component(res, right);
        assert_eq!(op(1_100, 100), Some(1_000));
        assert_eq!(op(1_100, 1_100), Some(0));
        assert_eq!(op(125, 25), Some(100));
        assert_eq!(op(300, 301), None);
        assert_eq!(op(1_000, 13_400), None);
    }

    #[test]
    pub fn test_multiplication() {
        let op = |res, right|Multiplication.get_left_component(res, right);
        assert_eq!(op(1_100, 100), Some(11));
        assert_eq!(op(1_100, 1_100), Some(1));
        assert_eq!(op(125, 25), Some(5));
        assert_eq!(op(0, 301), Some(0));
        assert_eq!(op(15, 10), None);
    }
}