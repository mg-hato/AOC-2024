#[test]
pub fn test_arguments_processing() {
    use crate::arguments;

    // shorthand to turn Vec<&str> into Vec<String>
    let make = |v: Vec<&str>|arguments::Arguments::new(
        &v.iter().map(|&s|s.to_string()).collect()
    );

    // shorthand to make arguments by giving explicit values (for expected results)
    let make_exp = |file, day, ip2|arguments::Arguments{
        optional_filepath: file,
        optional_day: day,
        is_part_2: ip2
    };
    
    // shorthand to turn &str -> Some(String)
    let some = |s: &str|Some(s.to_string());

    for (act, exp) in [
        (make(vec!["Program", "ok", "--p2"]), make_exp(None, None, true)),
        (make(vec!["Program", "--f", "--p2"]), make_exp(some("--p2"), None, true)),
        (make(vec!["Program", "--d", "--p2"]), make_exp(None, None, true)),
        (make(vec!["Program", "--alt", "--d", "ok"]), make_exp(None, None, true)),
        (make(vec!["Program", "--alt", "--d", "63", "--p2"]), make_exp(None, Some(63), true)),
        (make(vec!["Program", "--d", "10", "--f", "input.txt"]), make_exp(some("input.txt"), Some(10), false)),
        (make(vec!["Program", "--i", "A.txt", "--f", "B.txt"]), make_exp(some("A.txt"), None, false)),
    ] {
        assert_eq!(act, exp);
    }
}