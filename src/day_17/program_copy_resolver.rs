use crate::{answer::{Answer, DisplayableAnswer}, solver::Solve};

use super::{model::ProgramInformation, program_simulator::ProgramSimulator};


/// Solves for value of register A that would produce the output matching the starting program under certain
/// assumptions:
/// - ADV assumptions:
///   1. There is exactly one ADV instruction
///   2. At each iteration, it divides register A by a consant (i.e. literal operand)
///   3. The constant reduces register A (i.e. NOT literal operand 0, which does division by 1)
/// - JNZ assumptions:
///   1. There is exactly one JNZ instruction
///   2. The JNZ instruction is last instruction of the program
///   3. If jump takes place, it updates program counter to 0 (start)
/// - predictability of output values assumption: at each iteration of the program,
/// values of registers B and C from previous iterations are irrelevant. This means that even if
/// a value of register B or C does go to the output, we can reproduce it just by having register A's value
/// from the start of the iteration by following instruction sequence up to to `out` instruction.
/// 
/// These assumptions allow for solving this problem by starting from the last value in the output working backwards,
/// solving register A's value starting from its most significant bits (last output value) working our way to
/// the least significant bits (first output value).
pub struct ProgramCopyResolver {
    simulator: ProgramSimulator,
}

mod error {
    use crate::helper::display::vector_display;

    const PREFIX: &str = "[ProgramCopyResolver]";

    fn not_suitable() -> String {
        vector_display(&vec![
            format!("This violates the solving conditions."),
            format!("The current solver is not suitable for resolving initial value of register A."),
        ], " ")
    }

    pub fn invalid_program_length(length: usize) -> String {
        format!("{} program has length of {}. An even number length is required.", PREFIX, length)
    }

    pub fn exactly_n_instructions_required(n: usize, actual: usize, inst: &str) -> String {
        format!("{} program has {} {} instructions where exactly {} of these is expected. {}",
            PREFIX, actual, inst, n, not_suitable())
    }
    
    pub fn combo_operand(combo_op: u64, inst: &str) -> String {
        format!("{} program has unusuitable combo operand {} on instruction {}. {}",
            PREFIX, combo_op, inst, not_suitable())
    }

    pub fn jnz_not_last() -> String {
        format!("{} jnz instruction needs to be last. {}", PREFIX, not_suitable())
    }

    pub fn unknown_value_output(position: usize) -> String {
        vector_display(&vec![
            format!("{} out instruction at position {}", PREFIX, position),
            format!("will output a value that depends on the previous iteration of the program."),
            not_suitable(),
        ], " ")   
    }

    pub fn no_solution() -> String {
        format!("{} no solution has been found...", PREFIX)
    }
}

impl ProgramCopyResolver {
    pub fn new(max_runtime: usize) -> ProgramCopyResolver {
        ProgramCopyResolver { simulator: ProgramSimulator::new(max_runtime) }
    }

    fn get_instructions(program_info: &ProgramInformation, opcode: u64) -> Vec<(u64, u64)> {
        let mut i = 0;
        let mut insts = vec![];
        while i + 1 < program_info.program.len() {
            if program_info.program[i] == opcode {
                insts.push((program_info.program[i], program_info.program[i+1]));
            }
            i += 2;
        }
        insts
    }

    /// Checks ADV assumptions
    fn check_adv_assumptions(program_info: ProgramInformation) -> Result<ProgramInformation, String> {
        let adv_insts = Self::get_instructions(&program_info, 0);

        // exactly one
        if adv_insts.len() != 1 {
            return Err(error::exactly_n_instructions_required(1, adv_insts.len(), "adv"));
        }

        let (_, combo_op) = adv_insts[0];
        // non-zero literal operand
        if combo_op > 3 || combo_op == 0 {
            return Err(error::combo_operand(combo_op, "adv"));
        }
        Ok(program_info)
    }


    /// Checks JNZ assumptions
    fn check_jnz_assumptions(program_info: ProgramInformation) -> Result<ProgramInformation, String> {
        let jnz_insts = Self::get_instructions(&program_info, 3);
        
        // exactly one
        if jnz_insts.len() != 1 {
            return Err(error::exactly_n_instructions_required(1, jnz_insts.len(), "jnz"));
        }

        // jumps to start
        let (_, combo_op) = jnz_insts[0];
        if combo_op != 0 {
            return Err(error::combo_operand(combo_op, "jnz"));
        }

        // JNZ is last instruction
        if program_info.program[program_info.program.len() - 2] != 3 {
            return Err(error::jnz_not_last());
        }

        Ok(program_info)
    }

    /// Checks the predictability of the output values assumption
    fn check_output_predictability_assumption(program_info: ProgramInformation) -> Result<ProgramInformation, String> {

        // unknown: the value from previous iteration
        // derived: derived from A and/or literal operands
        #[derive(PartialEq, Eq, Clone, Copy)]
        enum RegStatus { Unknown, Derived } 
        use RegStatus::*;

        let (mut status_b, mut status_c) = (Unknown, Unknown);
        let mut i = 0;
        while  i + 1 < program_info.program.len() {
            // instructions to ignore: adv, bxl, jnz
            // - adv: due to solving conditions on adv it only uses literal operands and affects register A
            // - bxl: it is a XOR with a literal operand, does not change register B's status
            // - jnz: it's a jump dependant on A, does not affect register B nor C

            let opcode = program_info.program[i];
            // special case: bxc (opcode 4)
            // if both B and C are derived, C stays derived, o/w register C becomes unknown
            if opcode == 4 {
                status_c = if status_b == Derived && status_c == Derived { Derived } else { Unknown };
            }

            // otherwise: bst, out, bdv, cdv (opcodes below) are all combo operand based instructions
            else if vec![2, 5, 6, 7].contains(&opcode) {
                let resolved_operand_status = match program_info.program[i + 1] {
                    5 => status_b, // from register B: propagates its status
                    6 => status_c, // from register C: propagates its status
                    _ => Derived, // from register A or literal operand: Derived
                };

                // bst(2) or bdv(6) will write to register B
                if opcode == 2 || opcode == 6 { status_b = resolved_operand_status; }

                // cdv(7) will write to register C
                else if opcode == 7 { status_c = resolved_operand_status; }

                // out(5) will write to the output: if an "unknown" value is written to the output
                // then we cannot use our backward resolution of register A
                else if opcode == 5 && resolved_operand_status == Unknown {
                    return Err(error::unknown_value_output(i))
                }
            }
             
            i += 2;
        }
        
        Ok(program_info)
    }

    /// Gets the constant value by which register A is divided each loop of the program
    fn get_iteration_divisor_constant(program_info: &ProgramInformation) -> u64 {
        let (_, x) = Self::get_instructions(program_info, 0)[0];
        2u64.pow(x as u32)
    }

    /// Check that the program sequence satisfies solving assumptions
    fn satisfies_solving_assumptions(program_info: ProgramInformation) -> Result<ProgramInformation, String> {
        // Check the program sequence length
        if program_info.program.len() % 2 != 0 {
            return Err(error::invalid_program_length(program_info.program.len()));
        }

        Ok(program_info)
            .and_then(Self::check_adv_assumptions)
            .and_then(Self::check_jnz_assumptions)
            .and_then(Self::check_output_predictability_assumption)
    }

    /// returns true if the provided `output` vector is a suffix of the `program` vector
    fn is_suffix(program_info: &ProgramInformation, output: &Vec<u64>) -> bool {
        if program_info.program.len() < output.len() {
            return false;
        }
        let mut i = 1;
        while i <= output.len() {
            if output[output.len() - i] != program_info.program[program_info.program.len() - i] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

impl Solve<ProgramInformation> for ProgramCopyResolver {
    fn solve(&self, input: ProgramInformation) -> Result<Answer, String> {
        let mut program_info = match Self::satisfies_solving_assumptions(input) {
            Err(e) => return Err(e),
            Ok(pi) => pi,
        };

        let iteration_divisor = Self::get_iteration_divisor_constant(&program_info);
        
        let mut candidates = vec![];
        for initial_candidate in 1..iteration_divisor {
            candidates.push(initial_candidate);
        }

        let mut solutions = vec![];

        while !candidates.is_empty() {
            let candidate = candidates.pop().unwrap();
            program_info.register_a = candidate;

            let output = match self.simulator.run_program(&program_info) {
                Err(e) => return Err(e),
                Ok(out) => out,
            };
            
            if output == program_info.program { solutions.push(candidate); }
            
            else if Self::is_suffix(&program_info, &output) {
                for least_significant_bits in 0..iteration_divisor {
                    
                    let next_candidate = candidate.checked_mul(iteration_divisor)
                            .map(|value|value + least_significant_bits);

                    if next_candidate.is_some() {
                        candidates.push(next_candidate.unwrap());
                    }
                }
            }
        }

        solutions.into_iter().min().ok_or_else(error::no_solution).map(DisplayableAnswer::new)
    }
}