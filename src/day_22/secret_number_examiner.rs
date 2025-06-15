use crate::{answer::DisplayableAnswer, day_22::{model::Numbers, secret_number_transform::SecretNumberTransform}, solver::Solve};


/// Given a sequence of initial secret numbers, examines
/// what will be `n`-th evolution of each secret number
/// and returns their sum
pub struct SecretNumberExaminer {
    n: usize,
}

impl SecretNumberExaminer {
    pub fn new(n: usize) -> SecretNumberExaminer {
        SecretNumberExaminer { n }
    }
}

mod error {
    pub fn overflow() -> String {
        format!("overflow occurred")
    }
}

impl Solve<Numbers> for SecretNumberExaminer {
    fn solve(&self, input: Numbers) -> Result<crate::answer::Answer, String> {
        let Numbers(numbers) = input;
        let transform = SecretNumberTransform::default();
        
        numbers.into_iter().map(|secret|transform.iterative_evolve(secret, self.n))
            .try_fold(0u64, |acc, num|acc.checked_add(num).ok_or_else(error::overflow))
            .map(DisplayableAnswer::new)
    }
}