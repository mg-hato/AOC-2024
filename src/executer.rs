use crate::answer::Answer;

pub trait Execute {
    fn execute(&self, input_filepath: &str) -> Result<Answer, String>;
}

pub type Executer = Box<dyn Execute>;