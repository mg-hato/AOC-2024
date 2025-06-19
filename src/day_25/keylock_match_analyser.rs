use crate::{answer::DisplayableAnswer, day_25::model::KeyLockSchematics, solver::Solve};


pub struct KeyLockMatchAnalyser;

impl Solve<KeyLockSchematics> for KeyLockMatchAnalyser {
    fn solve(&self, input: KeyLockSchematics) -> Result<crate::answer::Answer, String> {
        Ok(DisplayableAnswer::new(0))
    }
}