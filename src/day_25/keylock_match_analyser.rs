use crate::{answer::DisplayableAnswer, day_25::{keylock_converter::KeyLockConverter, model::KeyLockSchematics}, solver::Solve};


pub struct KeyLockMatchAnalyser;

impl Solve<KeyLockSchematics> for KeyLockMatchAnalyser {
    fn solve(&self, input: KeyLockSchematics) -> Result<crate::answer::Answer, String> {
        KeyLockConverter::transform(input)
            .map(|(locks, keys)|{
                let mut key_lock_matches = 0u32;
                for lock in locks {
                    for key in keys.iter() {
                        if lock.key_fits(key) {
                            key_lock_matches += 1;
                        }
                    }
                }
                key_lock_matches
            }).map(DisplayableAnswer::new)
    }
}