

fn find_and_get_argument_at_offset(keywords: &[&str], args: &Vec<String>, offset: usize) -> Option<String> {
    let str_keywords = keywords
        .iter()
        .map(|keyword| keyword.to_string())
        .collect::<Vec<_>>();

    args.iter()
        .enumerate()
        .filter(|(_, arg)| str_keywords.contains(arg))
        .map_while(|(i, _)| args.get(i + offset))
        .collect::<Vec<_>>()
        .first()
        .map(|&s| s.clone())
}

fn is_part_2(args: &Vec<String>) -> bool {
    find_and_get_argument_at_offset(&["--p2","--alt"], args, 0)
        .is_some()
}

fn try_extract_day(args: &Vec<String>) -> Option<u8> {
    find_and_get_argument_at_offset(&["--d", "--p"], args, 1)
        .and_then(|argument| argument.parse::<u8>().ok())
}

fn try_get_input_file(args: &Vec<String>) -> Option<String> {
    find_and_get_argument_at_offset(&["--f", "--i"], args, 1)
}

#[derive(Eq, PartialEq, Debug)]
pub struct Arguments {
    pub optional_filepath: Option<String>,
    pub optional_day: Option<u8>,
    pub is_part_2: bool
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Arguments {
        Arguments {
            optional_filepath: try_get_input_file(args),
            optional_day: try_extract_day(args),
            is_part_2: is_part_2(args)
        }
    }

}