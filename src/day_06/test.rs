#[cfg(test)]
pub mod suite {
    use crate::{answer::DisplayableAnswer, day_06::{adjusted_loop_detector::AdjustedLoopDetector, direction::Direction, guard_state::GuardState, loop_detector::LoopDetector, make_pipeline, map_analyser::MapAnalyser, next_state::NextState, optimised_caching_loop_detector::OptimisedCachingLoopDetector}, executer::Execute, helper::position::UPosition, testing::get_verified_result_ok};


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

    fn guard_state(row: usize, col: usize, dir: Direction) -> GuardState {
        GuardState::new(UPosition::new((row, col)), dir)
    }

    #[test]
    pub fn test_optimised_caching_loop_detector() {
        let pipeline = make_pipeline(false).unwrap();
        let lab =  get_verified_result_ok(&pipeline, REL_FILEPATHS[0]);
       
        let mut map_analyser = MapAnalyser::new(lab).unwrap();
        let mut optimised_caching_loop_detector =  OptimisedCachingLoopDetector::new(&mut map_analyser);
        for (input, expected) in [
            // Both: waltzing upwards in column 0 until out
            (guard_state(0, 0, Direction::Up), NextState::Out),
            (guard_state(7, 0, Direction::Up), NextState::Out),

            // There is a block at 8, 0 i.e. immediately in front
            (guard_state(9, 0, Direction::Up), NextState::Next(guard_state(9, 0, Direction::Right))),

            // All hit the obstacle @0, 4, but from different directions
            (guard_state(3, 4, Direction::Up), NextState::Next(guard_state(1, 4, Direction::Right))),
            (guard_state(9, 4, Direction::Up), NextState::Next(guard_state(1, 4, Direction::Right))),
            (guard_state(0, 0, Direction::Right), NextState::Next(guard_state(0, 3, Direction::Down))),
            (guard_state(0, 9, Direction::Left), NextState::Next(guard_state(0, 5, Direction::Up))),
            
            // Same as above, but for obstacle @9, 6
            (guard_state(3, 6, Direction::Down), NextState::Next(guard_state(8, 6, Direction::Left))),
            (guard_state(9, 1, Direction::Right), NextState::Next(guard_state(9, 5, Direction::Down))),
            (guard_state(9, 9, Direction::Left), NextState::Next(guard_state(9, 7, Direction::Up))),
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
        let mut adjusted_loop_detector = AdjustedLoopDetector::new(&mut optimised_caching_loop_detector, UPosition::new((4, 4)));

        for (input, expected) in [
            // Same as before, except for 9, 4 Up
            (guard_state(0, 0, Direction::Up), NextState::Out),
            (guard_state(7, 0, Direction::Up), NextState::Out),
            (guard_state(9, 0, Direction::Up), NextState::Next(guard_state(9, 0, Direction::Right))),
            (guard_state(3, 4, Direction::Up), NextState::Next(guard_state(1, 4, Direction::Right))),
            (guard_state(0, 0, Direction::Right), NextState::Next(guard_state(0, 3, Direction::Down))),
            (guard_state(0, 9, Direction::Left), NextState::Next(guard_state(0, 5, Direction::Up))),
            (guard_state(3, 6, Direction::Down), NextState::Next(guard_state(8, 6, Direction::Left))),
            (guard_state(9, 1, Direction::Right), NextState::Next(guard_state(9, 5, Direction::Down))),
            (guard_state(9, 9, Direction::Left), NextState::Next(guard_state(9, 7, Direction::Up))),
            // This one would hit the obstacle: (guard_state(9, 4, Direction::Up), NextState::Next(guard_state(1, 4, Direction::Right))),

            // All of the below ones hit the new obstacle @4, 4
            (guard_state(9, 4, Direction::Up), NextState::Next(guard_state(5, 4, Direction::Right))),
            (guard_state(1, 4, Direction::Down), NextState::Next(guard_state(3, 4, Direction::Left))),
            (guard_state(4, 6, Direction::Left), NextState::Next(guard_state(4, 5, Direction::Up))),
            (guard_state(4, 5, Direction::Left), NextState::Next(guard_state(4, 5, Direction::Up))),
            (guard_state(4, 3, Direction::Right), NextState::Next(guard_state(4, 3, Direction::Down))),
            
            // Few more potential crosses with the obstacle, but not actually
            (guard_state(4, 9, Direction::Left), NextState::Next(guard_state(4, 8, Direction::Up))),
            (guard_state(4, 5, Direction::Right), NextState::Next(guard_state(4, 6, Direction::Down))),
            (guard_state(5, 4, Direction::Down), NextState::Out),
            (guard_state(7, 4, Direction::Down), NextState::Out),

        ] {
            assert_eq!(adjusted_loop_detector.next_state(input).unwrap(), expected)
        }
    }
}