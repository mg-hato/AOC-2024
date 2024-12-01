#[cfg(test)]
use crate::{pipelined_executer::PipelinedExecuter,answer::Answer};

#[cfg(test)]
use std::fmt::{Display, Debug};


/// A helper function only to be used for test purposes.
/// It helps resolve the file relative path, starting from root of the project,
/// which would be the folder that contains `src`. Performs assertions along the way.
#[cfg(test)]
pub fn resolve_filepath(root_relative_path: &str) -> String {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(root_relative_path);
    let optional_str = path.to_str();
    assert!(path.is_file() && optional_str.is_some());
    String::from(optional_str.unwrap())
}

/* PARSING SECTION */

/// Helper method to read given file, assert successful read and parse the read lines.
/// Returns parsed result (successful or not)
#[cfg(test)]
fn get_parsed_result<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> Result<T, String>
where T: Eq + Display + Clone + Debug {
    use crate::{parser::Parse, reader::Read};
    let input_file_path = &resolve_filepath(root_relative_path);
    let lines = pipeline.read(input_file_path);
    assert!(lines.is_ok());
    pipeline.parse(lines.unwrap())
}

/// Helper method that reads the file, parses the lines and returns an underlying value of succesful parsed result.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_parsed_result_ok<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> T
where T: Eq + Display + Clone + Debug {
    let parsed = get_parsed_result(pipeline, root_relative_path);
    assert!(parsed.is_ok());
    parsed.unwrap()
}

/// Helper method that reads the file, parses the lines successfully and compares parsed value against expected.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn test_parsing<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str, expected: T)
where T: Eq + Display + Clone + Debug {
    let result = get_parsed_result_ok(pipeline, root_relative_path);
    assert_eq!(result, expected);
}

/// Helper method that reads the file and returns a parsing error.
/// Performs all assertions along the way.
#[cfg(test)]
#[allow(dead_code)]
pub fn get_parsing_error<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> String
where T: Eq + Display + Clone + Debug {
    let parsed = get_parsed_result(pipeline, root_relative_path);
    assert!(parsed.is_err());
    parsed.unwrap_err()
}


/* VERIFIER SECTION */

/// Helper method that reads the file, parses the lines successfully and returns verified result.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_verified_result<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> Result<T, String>
where T: Eq + Display + Clone + Debug {
    use crate::verifier::Verify;
    pipeline.verify(get_parsed_result_ok(pipeline, root_relative_path))
}

/// Helper method that reads the file, parses the lines successfully and returns successfully verified result.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_verified_result_ok<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> T
where T: Eq + Display + Clone + Debug {
    let verified = get_verified_result(pipeline, root_relative_path);
    assert!(verified.is_ok());
    verified.unwrap()
}


/// Helper method that reads the file, parses the lines successfully and returns verification error.
/// Performs all assertions along the way.
#[cfg(test)]
#[allow(dead_code)]
pub fn get_verification_error<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> String
where T: Eq + Display + Clone + Debug {
    let verified = get_verified_result(pipeline, root_relative_path);
    assert!(verified.is_err());
    verified.unwrap_err()
}


/* SOLVER SECTION */

/// Helper method that reads the file, parses the lines successfully,
/// verifies the parsed input and returns an answer result.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_answer<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> Result<Answer, String>
where T: Eq + Display + Clone + Debug {
    use crate::solver::Solve;
    pipeline.solve(get_verified_result_ok(pipeline, root_relative_path))
}


/// Helper method that reads the file, parses the lines successfully,
/// verifies the parsed input and returns a successful answer.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_answer_ok<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> Answer
where T: Eq + Display + Clone + Debug {
    let answer = get_answer(pipeline, root_relative_path);
    assert!(answer.is_ok());
    answer.unwrap()
}


/// Helper method that reads the file, parses the lines successfully,
/// verifies the parsed input and returns an error produced by the solver.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn get_answer_error<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str) -> String
where T: Eq + Display + Clone + Debug {
    let answer = get_answer(pipeline, root_relative_path);
    assert!(answer.is_err());
    answer.unwrap_err()
}


/// Helper method that reads the file, parses the lines successfully,
/// verifies the parsed input and returns an error produced by the solver.
/// Performs all assertions along the way.
#[cfg(test)]
pub fn test_whole_flow<T>(pipeline: &PipelinedExecuter<T>, root_relative_path: &str, expected: Answer)
where T: Eq + Display + Clone + Debug {
    let answer = get_answer_ok(pipeline, root_relative_path);
    assert_eq!(answer.report(), expected.report());
}