use std::fmt::{Debug, Display};
use crate::{answer::Answer, executer::Execute, helper::result::zip, parser::{Parse, Parser}, reader::{Read, Reader}, solver::{Solve, Solver}, verifier::{Verifier, Verify}};

/// A pipelined executer on generic `T`. It captures the idea of
/// reading, parsing, verifying and solving, and it implements
/// all those corresponding traits as well.
pub struct PipelinedExecuter<T> where T: Eq + Display + Clone + Debug {
    reader: Reader,
    parser: Parser<T>,
    verifier: Verifier<T>,
    solver: Solver<T>,
}

impl <T> PipelinedExecuter<T> where T: Eq + Display + Clone + Debug + 'static {
    pub fn new<R, P, V, S>(reader: R, parser: P, verifier: V, solver: S) -> PipelinedExecuter<T>
    where R: Read + 'static, P: Parse<T> + 'static, V: Verify<T> + 'static, S: Solve<T> + 'static,
    {
        PipelinedExecuter{
            reader: Box::new(reader),
            parser: Box::new(parser),
            verifier: Box::new(verifier),
            solver: Box::new(solver),
        }
    }
}

impl <T> Read for PipelinedExecuter<T>  where T: Eq + Display + Clone + Debug {
    fn read(&self, input_file_path: &str) -> Result<crate::reader::VecLine, String> {
        self.reader.read(input_file_path)
    }
}

impl <T> Parse<T> for PipelinedExecuter<T>  where T: Eq + Display + Clone + Debug {
    fn parse(&self, lines: crate::reader::VecLine) -> Result<T, String> {
        self.parser.parse(lines)
    }
}

impl <T> Verify<T> for PipelinedExecuter<T>  where T: Eq + Display + Clone + Debug {
    fn verify(&self, input: T) -> Result<T, String> {
        self.verifier.verify(input)
    }
}

impl <T> Solve<T> for PipelinedExecuter<T>  where T: Eq + Display + Clone + Debug {
    fn solve(&self, input: T) -> Result<Answer, String> {
        self.solver.solve(input)
    }
}

impl <T> Execute for PipelinedExecuter<T> where T: Eq + Display + Clone + Debug {
    fn execute(&self, input_file_path: &str) -> Result<Answer, String> {
        let input_lines = self.reader.read(input_file_path);
        let parsed_input = input_lines.and_then(|lines| self.parser.parse(lines));
        let verified_input = parsed_input.and_then(|input| self.verifier.verify(input));
        let solution = verified_input.and_then(|input| self.solver.solve(input));
        solution
    }
}

/// Providing all components in their `Result<_,String>` form, where creation of each
/// component may fail, this helper function just yields a pipeline executer in
/// its `Result<_,String>` form. The returned result is an `Ok` value if all components
/// are `Ok` values. Otherwise, the first `Err` value is returned, defined by the order
/// of input parameters.
pub fn try_make_pipeline<T, R, P, V, S>(
    reader: Result<R, String>,
    parser: Result<P, String>,
    verifier: Result<V, String>,
    solver: Result<S, String>,
) -> Result<PipelinedExecuter<T>, String>
where T: Eq + Display + Clone + Debug + 'static,
    R: Read + 'static,
    P: Parse<T> + 'static,
    V: Verify<T> + 'static,
    S: Solve<T> + 'static,
{
    let reader_parser = zip(reader, parser, |r,p|(r,p));
    let verifier_solver = zip(verifier, solver, |v,s|(v,s));
    zip(reader_parser, verifier_solver, |(r,p),(v,s)|PipelinedExecuter::new(r,p,v,s))
}