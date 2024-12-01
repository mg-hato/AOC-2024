use std::collections::HashMap;
use crate::executer::Execute;
use crate::helper::re::get_captures;
use crate::{answer::Answer, executer::Executer};

pub struct ExecuterManager {
    registered_executers: HashMap<String, Executer>,
    
    day_pattern_re: regex::Regex
}

impl ExecuterManager {
    const PREFIX : &str = "[ExecuterManager]";

    pub fn new() -> Result<ExecuterManager, String> {
        let day_pattern = regex::Regex::new(r"d(?:ay)?(?:\.|-|_)?(\d{2})")
            .map_err(|e| format!("{} compilation of day pattern regex failed with error '{}'", Self::PREFIX, e));

        day_pattern.map(|dp| ExecuterManager{ day_pattern_re: dp, registered_executers: HashMap::new() })
    }

    /// Returns a unique key identifier for executer in question
    fn try_get_key(day: u8, is_part_2: bool) -> Result<String, String> {
        match day {
            d if 1 <= d && d <= 25 => Ok(format!("D{:02}{}", d, if is_part_2 { "-P2" } else { "" })),
            _ => Err(format!("{} invalid day number: {}", Self::PREFIX, day))
        }
    }

    /// Tries to resolve the day number from the given input filepath
    fn try_resolve_day(&self, input_filepath: &str) -> Result<u8, String> {
        match get_captures(&self.day_pattern_re, input_filepath) {
            Some(ref v) if v.len() == 2 => {
                let day = &v[1];
                day.parse::<u8>()
                    .map_err(|e|e.to_string())
            }
            _ => Err(format!("{} day not provided, failed to resolve day from filename: '{}'", Self::PREFIX, input_filepath)),
        }
    }

    fn handle_overwrite(self, allow_overwrite: bool, key: &String) -> Result<Self, String> {
        match allow_overwrite {
            true => Ok(self),
            false => Err(format!("{} attempted overwrite on registered key '{}'", Self::PREFIX, key))
        }
    }

    /// Registers an executer for specific problem.
    /// - `day`: a number representing the associated day of the problem. Valid values: 1 to 25, inclusive.
    /// - `is_part_2`: whether the executer is to be associated with part two for the given day
    /// - `executer`: executer to be registered
    /// - `allow_overwrite`: a flag that tells whether overwrite is allowed. If `true` and an executer is being registered against
    /// already existing key-value entry, registration will fail with an error
    /// 
    /// Returns `Result<Self, String>`: if registration failed, the error message is returned, o/w it returns self.
    /// Main reason for the failure is if the `day` number is out of its valid range.
    pub fn register<EXE>(mut self, day: u8, is_part_2: bool, executer: EXE, allow_overwrite: bool) -> Result<Self, String>
    where EXE: Execute + 'static
    {
        ExecuterManager::try_get_key(day, is_part_2)
            .and_then(|key|match self.registered_executers.insert(key.clone(), Box::new(executer)) {
                Some(_) => self.handle_overwrite(allow_overwrite, &key),
                None => Ok(self),
            })
    }

    /// A shorthand when the underlying executer is wrapped in `Result<_, String>`.
    /// Effectively does the same as `register`, but handles the "unpacking"
    pub fn try_register<EXE>(self, day: u8, is_part_2: bool, executer: Result<EXE, String>, allow_overwrite: bool)
    -> Result<Self, String>
    where EXE: Execute + 'static {
        executer.and_then(|exe|self.register(day, is_part_2, exe, allow_overwrite))
    }

    /// Tries to find a registered executer to execute against the input file.
    /// - `input_filepath`: filepath to the input file.
    /// - `day`: optional number of the day to execute. If `None` it will try to resolve the day number using `input_filepath`.
    /// - `is_part_2`: a boolean representing if the part 2 of the problem is to be solved.
    /// 
    /// If day is not provided, certain patterns in the filepath can be used to deduce the day. The patterns are the following:
    /// `dXY`,`d.XY`,`d-XY`,`d_XY`,`dayXY`,`day.XY`,`day-XY`,`day_XY` where `XY` are two digits representing the day number.
    /// If the input filepath contains the any of the pattern, they will be detected and used when `day` is `None`.
    /// There are no guarantees which pattern will be used if multiple are present, e.g. for filepath `d05/input.d-12.txt`
    /// the day can be resolved either to day 5 or day 12.
    pub  fn try_execute_executer(&self, input_filepath: &str, day: Option<u8>, is_part_2: bool) -> Result<Answer, String> {
        day.map_or_else(||self.try_resolve_day(input_filepath), Ok)
            .and_then(|day|ExecuterManager::try_get_key(day, is_part_2))
            .and_then(|key|self.registered_executers
                .get(&key)
                .ok_or(format!("There is no registered executer for key '{}'", key)))
            ?.execute(input_filepath)
    }
}
