#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_02::{make_pipeline, models::{LevelReport, LevelReports}}, solver::Solve, testing::{test_parsing, test_whole_flow}};

    
    const REL_FILEPATHS: &[&str] = &[
        "src/day_02/test/example.txt", // Example given on AOC24
        "src/day_02/test/another_example.txt",
    ];

    #[test]
    pub fn test_parser_example() {
        test_parsing(&make_pipeline(false).unwrap(), REL_FILEPATHS[0], LevelReports::new(vec![
            LevelReport::new(vec![7, 6, 4, 2, 1]),
            LevelReport::new(vec![1, 2, 7, 8, 9]),
            LevelReport::new(vec![9, 7, 6, 2, 1]),
            LevelReport::new(vec![1, 3, 2, 4, 5]),
            LevelReport::new(vec![8, 6, 4, 4, 1]),
            LevelReport::new(vec![1, 3, 6, 7, 9]),
        ]));
    }
    

    #[test]
    pub fn test_parser_another_example() {
        test_parsing(&make_pipeline(false).unwrap(), REL_FILEPATHS[1], LevelReports::new(vec![
            LevelReport::new(vec![1, 2, 3, 4, 5, 6]),
            LevelReport::new(vec![1_000, 1]),
            LevelReport::new(vec![33]),
        ]));
    }

    #[test]
    pub fn test_whole_flow_pt1_example() {
        test_whole_flow(
            &make_pipeline(false).unwrap(),
            REL_FILEPATHS[0],
            DisplayableAnswer::new(2usize));
    }

    
    #[test]
    pub fn test_whole_flow_pt2_example() {
        test_whole_flow(
            &make_pipeline(true).unwrap(),
            REL_FILEPATHS[0],
            DisplayableAnswer::new(4usize));
    }

    fn make_single_report(levels: Vec<u32>) -> LevelReports {
        LevelReports::new(vec![LevelReport::new(levels)])
    }

    #[test]
    pub fn test_pt_2_solver_with_various_sequences() {
        let pipeline = make_pipeline(true).unwrap();
        for (levels, exp) in [
            (vec![], 1usize), // Empty report is always safe
            (vec![100], 1usize), // Single level report is always safe
            (vec![1, 100, 3], 1usize), // Report is not safe, but it becomes safe if 100 is removed
            (vec![100, 1, 3], 1usize), // Same as above
            (vec![4, 1, 100], 1usize), // Same as above

            // Nothing can be done here:
            // one of the 5's needs to go to ensure strictly increasing trend
            // but that leaves 9, which is too far apart (movement of up 4)
            (vec![4, 5, 5, 9], 0usize),
            (vec![4, 5, 5, 8], 1usize), // Remove any of the 5's
            (vec![1, 0, 2, 3, 4], 1usize), // Remove 0
            (vec![1, 4, 2, 3, 4], 1usize), // Remove the first 4

            // Nothing can be done. 3 and 4 at the end form increasing trend.
            // Removing anything left of 3 will still form a decreasing trend with 3 and its left neighbour.
            // Removing either 3 or 4 at the end will not fix the zig-zag trend at the start: 5,4,5
            (vec![5, 4, 5, 3, 4], 0usize),

            // Nothing can be done here.
            // Clearly, 100 should be removed because it is too far apart from any other level.
            // However, doing so will create a zig-zag sequence that we cannot get rid of: ...,7,6,10
            (vec![1, 2, 4, 7, 100, 6, 10], 0usize),

        ] {
            let answer = pipeline.solve(make_single_report(levels)).unwrap();
            assert_eq!(answer.report(), DisplayableAnswer::new(exp).report());
        }
    }
}