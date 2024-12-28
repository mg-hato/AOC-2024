#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_04::make_pipeline, executer::Execute};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_04/test/example.txt", // Example given on AOC24
    ];

    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(18).report());
    }

    

    #[test]
    pub fn test_whole_flow_pt2_example() {
        let pipeline = make_pipeline(true).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(9).report());
    }
}