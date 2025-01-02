#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, testing::get_verification_error, day_05::make_pipeline, executer::Execute};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_05/test/example.txt", // Example given on AOC24
        "src/day_05/test/cycle_example.txt",
        "src/day_05/test/multiple_example.txt",
        "src/day_05/test/duplicate_example.txt",
        "src/day_05/test/even_example.txt",
    ];

    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(143).report());
    }

    #[test]
    pub fn test_whole_flow_pt2_example() {
        let pipeline = make_pipeline(true).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(123).report());
    }

    #[test]
    pub fn test_whole_flow_pt1_cycle_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[1]).unwrap().report(), DisplayableAnswer::new(7).report());
    }

    #[test]
    pub fn test_whole_flow_pt2_cycle_example() {
        let pipeline = make_pipeline(true).unwrap();
        let res = pipeline.execute(REL_FILEPATHS[1]);
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert!(err.contains("[1,2,3,4,5]"))
    }

    #[test]
    pub fn test_whole_flow_pt1_multiple_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[2]).unwrap().report(), DisplayableAnswer::new(2).report());
    }

    #[test]
    pub fn test_whole_flow_pt2_multiple_example() {
        let pipeline = make_pipeline(true).unwrap();
        let res = pipeline.execute(REL_FILEPATHS[2]);
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert!(err.contains("[3,2,1]"))
    }

    #[test]
    pub fn test_verifier_duplicate_error() {
        let pipeline = &make_pipeline(false).unwrap();
        assert!(get_verification_error(pipeline, REL_FILEPATHS[3]).contains("duplicate"));
    }

    #[test]
    pub fn test_verifier_even_error() {
        let pipeline = &make_pipeline(false).unwrap();
        assert!(get_verification_error(pipeline, REL_FILEPATHS[4]).contains("even"));
    }
}