pub type Verifier<T> = Box<dyn Verify<T>>;

pub trait Verify<T>
{
    fn verify(&self, input: T) -> Result<T, String>;
}

pub struct TrivialVerifier;

impl TrivialVerifier {
    pub fn new<T>() -> TrivialVerifier { TrivialVerifier }
}

impl <T> Verify<T> for TrivialVerifier {
    fn verify(&self, input: T) -> Result<T, String> {
        Ok(input)
    }
}