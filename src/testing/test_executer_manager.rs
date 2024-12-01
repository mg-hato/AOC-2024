#[cfg(test)]
pub mod executer_manager_test_suite {
    use crate::{answer::{Answer, DisplayableAnswer}, executer::Execute, executer_manager::ExecuterManager};

    /// Creates an standard displayable answer implementation that reports string `value`
    fn str_report(value: &str) -> Answer {
        DisplayableAnswer::new(value.to_string())
    }

    /// An executer for testing purposes, simply receives a string value
    /// which it will return in `DisplayableAnswer` wrapper
    struct TestingExecuter{
        value: String
    }

    impl TestingExecuter {
        fn new(day: u8, is_part_2: bool) -> TestingExecuter {
            let value = format!("{}{}", day, if is_part_2 {"A"} else {""});
            TestingExecuter{value}
        }
    }
    
    impl Execute for TestingExecuter{
        fn execute(&self, _input_filepath: &str) -> Result<Answer, String> {
            Ok(str_report(&self.value))
        }
    }
    
    fn get_exec_manager() -> ExecuterManager {
        let manager = ExecuterManager::new();
        assert!(manager.is_ok());
        manager.unwrap()
    }

    /// Executer manager with trivial executers for day 3, 5, 11. Only day 5 is alternative/part 2
    fn get_filled_exec_manager() -> ExecuterManager {
        let mut manager = get_exec_manager();
        for (day, ip2) in [(3, false), (5, true), (11, false)] {
            let res = manager.register(day, ip2, TestingExecuter::new(day, ip2), false);
            assert!(res.is_ok());
            manager = res.unwrap();
        }
        manager
    }

    #[test]
    pub fn test_executer_manager_creation() {
        get_exec_manager();
    }

    #[test]
    pub fn test_executer_manager_different_patterns_day_resolution() {
        let manager = get_filled_exec_manager();
        
        // Try different patterns
        for input in [
            "input.d03",
            "day-03.input.txt",
            "problem.d.03.txt",
            "d_03/input.txt",
            "example-d03.txt"
            ] {
            let answer = manager.try_execute_executer(input, None, false);
            
            // it should resolve the day from input file
            // and then it should call executer, which trivially returns ok
            // but in particular, it should call executer for day 3, part 1,
            // which by design trivially returns reportable with solution "3"
            assert!(answer.is_ok());
            assert_eq!(answer.unwrap().report(), str_report("3").report());
        }
    }

    #[test]
    pub fn test_executer_manager_different_parts_for_same_day() {
        let manager = get_filled_exec_manager();
        let input = "day05/input.txt"; // should resolve to day 5 from the filepath

        // Part one report should be a failure, because no executer is registered for it
        let part_one_report =  manager.try_execute_executer(input, None, false);
        assert!(part_one_report.is_err());

        // Part two report should be fine, with value "5A"
        let part_two_report = manager.try_execute_executer(input, None, true);
        assert!(part_two_report.is_ok());
        let p2_report = part_two_report.unwrap();
        assert_eq!(p2_report.report(), str_report("5A").report());

        // It should also work if we explicitly pass in the day
        let part_two_report_again = manager.try_execute_executer(input, Some(5), true);
        assert!(part_two_report_again.is_ok());
        assert_eq!(p2_report.report(), part_two_report_again.unwrap().report());
    }

    #[test]
    pub fn test_executer_manager_registering() {
        let manager = get_exec_manager();
        
        // It should be fine to register, day 3 part 2 or day 25 part 1
        let res = manager.register(3, true, TestingExecuter::new(3, true), false);
        assert!(res.is_ok());
        let res = res.unwrap().register(25, false, TestingExecuter::new(25, false), false);
        assert!(res.is_ok());

        // It should FAIL registering, say, day 0 and 26
        assert!(get_exec_manager().register(0, true, TestingExecuter::new(0, true), false).is_err());
        assert!(get_exec_manager().register(26, false, TestingExecuter::new(26, false), false).is_err());
    }

    #[test]
    pub fn test_executer_manager_ignoring_day_resolution() {
        let manager = get_filled_exec_manager();
        let report = manager.try_execute_executer("day03/input.txt", Some(11), false);
        assert!(report.is_ok());

        // Although the filepath to input file seems like it is a day 3 input, we have explicitly selected day 11
        // So we expect the filepath-based day resolution will not take place and we get executer for day 11 part 1
        // (which prints "11" as the answer value)
        assert_eq!(report.unwrap().report(), str_report("11").report())
    }

    #[test]
    pub fn test_overwrite_detection() {
        // We already have day 5 part 2 registered, but we set the overwrite flag to true to allow this
        assert!(get_filled_exec_manager().register(5, true, TestingExecuter::new(5, true), true).is_ok());

        // Now, we do it again, but with overwrite flag set to false, we expect failure
        assert!(get_filled_exec_manager().register(5, true, TestingExecuter::new(5, true), false).is_err());
        
    }
}