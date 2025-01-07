#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_06::{adjusted_loop_detector::AdjustedLoopDetector, direction::Direction, guard_state::GuardState, loop_detector::LoopDetector, make_pipeline, map_analyser::MapAnalyser, next_state::NextState, optimised_caching_loop_detector::OptimisedCachingLoopDetector}, executer::Execute, testing::get_verified_result_ok};


    const REL_FILEPATHS: &[&str] = &[
        "src/day_06/test/example.txt", // Example given on AOC24
    ];


    #[test]
    pub fn test_whole_flow_pt1_example() {
        let pipeline = make_pipeline(false).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(41).report());
    }

    #[test]
    pub fn test_whole_flow_pt2_example() {
        let pipeline = make_pipeline(true).unwrap();
        assert_eq!(pipeline.execute(REL_FILEPATHS[0]).unwrap().report(), DisplayableAnswer::new(6).report());
    }
    #[test]
    pub fn test_optimised_caching_loop_detector() {
        let pipeline = make_pipeline(false).unwrap();
        let lab =  get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
       
        let mut map_analyser = MapAnalyser::new(lab).unwrap();
        let mut optimised_caching_loop_detector =  OptimisedCachingLoopDetector::new(&mut map_analyser);
        for (input, expected) in [
            // Both: waltzing upwards in column 0 until out
            (GuardState::new((0, 0), Direction::Up), NextState::Out),
            (GuardState::new((7, 0), Direction::Up), NextState::Out),

            // There is a block at (8, 0) i.e. immediately in front
            (GuardState::new((9, 0), Direction::Up), NextState::Next(GuardState::new((9, 0), Direction::Right))),

            // All hit the obstacle @(0, 4), but from different directions
            (GuardState::new((3, 4), Direction::Up), NextState::Next(GuardState::new((1, 4), Direction::Right))),
            (GuardState::new((9, 4), Direction::Up), NextState::Next(GuardState::new((1, 4), Direction::Right))),
            (GuardState::new((0, 0), Direction::Right), NextState::Next(GuardState::new((0, 3), Direction::Down))),
            (GuardState::new((0, 9), Direction::Left), NextState::Next(GuardState::new((0, 5), Direction::Up))),
            
            // Same as above, but for obstacle @(9, 6)
            (GuardState::new((3, 6), Direction::Down), NextState::Next(GuardState::new((8, 6), Direction::Left))),
            (GuardState::new((9, 1), Direction::Right), NextState::Next(GuardState::new((9, 5), Direction::Down))),
            (GuardState::new((9, 9), Direction::Left), NextState::Next(GuardState::new((9, 7), Direction::Up))),
        ] {
            let actual = optimised_caching_loop_detector.next_state(input);
            assert_eq!(actual.unwrap(), expected)
        }
    }

    

    #[test]
    pub fn test_adjusted_loop_detector_with_optimised_caching_underlying() {
        let pipeline = make_pipeline(false).unwrap();
        let lab =  get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
       
        let mut map_analyser = MapAnalyser::new(lab).unwrap();
        let mut optimised_caching_loop_detector =  OptimisedCachingLoopDetector::new(&mut map_analyser);
        let mut adjusted_loop_detector = AdjustedLoopDetector::new(&mut optimised_caching_loop_detector, (4, 4));

        for (input, expected) in [
            // Same as before, except for (9,4) Up
            (GuardState::new((0, 0), Direction::Up), NextState::Out),
            (GuardState::new((7, 0), Direction::Up), NextState::Out),
            (GuardState::new((9, 0), Direction::Up), NextState::Next(GuardState::new((9, 0), Direction::Right))),
            (GuardState::new((3, 4), Direction::Up), NextState::Next(GuardState::new((1, 4), Direction::Right))),
            (GuardState::new((0, 0), Direction::Right), NextState::Next(GuardState::new((0, 3), Direction::Down))),
            (GuardState::new((0, 9), Direction::Left), NextState::Next(GuardState::new((0, 5), Direction::Up))),
            (GuardState::new((3, 6), Direction::Down), NextState::Next(GuardState::new((8, 6), Direction::Left))),
            (GuardState::new((9, 1), Direction::Right), NextState::Next(GuardState::new((9, 5), Direction::Down))),
            (GuardState::new((9, 9), Direction::Left), NextState::Next(GuardState::new((9, 7), Direction::Up))),
            // This one would hit the obstacle: (GuardState::new((9, 4), Direction::Up), NextState::Next(GuardState::new((1, 4), Direction::Right))),

            // All of the below ones hit the new obstacle @(4,4)
            (GuardState::new((9, 4), Direction::Up), NextState::Next(GuardState::new((5, 4), Direction::Right))),
            (GuardState::new((1, 4), Direction::Down), NextState::Next(GuardState::new((3, 4), Direction::Left))),
            (GuardState::new((4, 6), Direction::Left), NextState::Next(GuardState::new((4, 5), Direction::Up))),
            (GuardState::new((4, 5), Direction::Left), NextState::Next(GuardState::new((4, 5), Direction::Up))),
            (GuardState::new((4, 3), Direction::Right), NextState::Next(GuardState::new((4, 3), Direction::Down))),
            
            // Few more potential crosses with the obstacle, but not actually
            (GuardState::new((4, 9), Direction::Left), NextState::Next(GuardState::new((4, 8), Direction::Up))),
            (GuardState::new((4, 5), Direction::Right), NextState::Next(GuardState::new((4, 6), Direction::Down))),
            (GuardState::new((5, 4), Direction::Down), NextState::Out),
            (GuardState::new((7, 4), Direction::Down), NextState::Out),

        ] {
            assert_eq!(adjusted_loop_detector.next_state(input).unwrap(), expected)
        }
    }
}