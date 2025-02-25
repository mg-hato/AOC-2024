use crate::{answer::{Answer, DisplayableAnswer}, helper::display::vector_display, solver::Solve};

use super::model::ProgramInformation;


pub struct ProgramSimulator {
    max_runtime: usize,
}

mod error {
    const PREFIX: &str = "[ProgramSimulator]";

    pub fn invalid_program_length(length: usize) -> String {
        format!("{} program has length of {}. An even number length is required.", PREFIX, length)
    }

    pub fn invalid_opcode(opcode: u64) -> String {
        format!("{} invalid opcode found: {}.", PREFIX, opcode)
    }

    pub fn invalid_combo_operand(operand: u64) -> String {
        format!("{} invalid combo operand value: {}.", PREFIX, operand)
    }

    pub fn maximum_runtime_reached(max_runtime: usize) -> String {
        format!("{} maximum runtime reached: {}.", PREFIX, max_runtime)
    }
}

impl ProgramSimulator {
    pub fn new(max_runtime: usize) -> ProgramSimulator {
        ProgramSimulator { max_runtime }
    }

    fn make_initial_program_state(program_information: &ProgramInformation) -> ProgramState {
        ProgramState {
            register_a: program_information.register_a,
            register_b: program_information.register_b,
            register_c: program_information.register_c,
            program_counter: 0,
        }
    }

    fn next(state: ProgramState, opcode: u64, operand: u64, output: &mut Vec<u64>) -> Result<ProgramState, String> {
        match opcode {
            0 => Self::adv(state, operand),
            1 => Self::bxl(state, operand),
            2 => Self::bst(state, operand),
            3 => Self::jnz(state, operand),
            4 => Self::bxc(state, operand),
            5 => Self::out(state, operand, output),
            6 => Self::bdv(state, operand),
            7 => Self::cdv(state, operand),
            _ => Err(error::invalid_opcode(opcode))
        }
    }

    fn resolve_combo_operand(state: ProgramState, operand: u64) -> Result<u64, String> {
        Ok(match operand {
            0 | 1 | 2 | 3 => operand,
            4 => state.register_a,
            5 => state.register_b,
            6 => state.register_c,
            _ => return Err(error::invalid_combo_operand(operand))
        })
    }

    fn adv(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        let numerator = state.register_a;
        let power = match Self::resolve_combo_operand(state, operand) {
            Ok(pwr) => pwr,
            Err(e) => return Err(e),
        };
        let result = if power >= 64 { 0 } else { numerator / 2u64.pow(power as u32) };
        Ok(ProgramState { program_counter: state.program_counter + 2, register_a: result, ..state })
    }

    fn bxl(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        let result = state.register_b ^ operand;
        Ok(ProgramState { program_counter: state.program_counter + 2, register_b: result, ..state })
    }

    fn bst(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        let result = match Self::resolve_combo_operand(state, operand) {
            Ok(resolved) => resolved % 8,
            Err(e) => return Err(e),
        };
        Ok(ProgramState { program_counter: state.program_counter + 2, register_b: result, ..state })
    }

    fn jnz(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        let next_program_counter = if state.register_a == 0 { state.program_counter + 2 }
        else { operand };
        Ok(ProgramState { program_counter: next_program_counter, ..state })
    }

    fn bxc(state: ProgramState, _operand: u64) -> Result<ProgramState, String> {
        let result = state.register_b ^ state.register_c;
        Ok(ProgramState { program_counter: state.program_counter + 2, register_b: result, ..state })
    }

    fn out(state: ProgramState, operand: u64, output: &mut Vec<u64>) -> Result<ProgramState, String> {
        match Self::resolve_combo_operand(state, operand) {
            Ok(output_num) => output.push(output_num % 8),
            Err(e) => return Err(e),
        };
        Ok(ProgramState { program_counter: state.program_counter + 2, ..state })
    }


    fn bdv(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        Self::adv(state, operand)
            .map(|state|state.register_a)
            .map(|result|ProgramState { program_counter: state.program_counter + 2, register_b: result, ..state })
    }


    fn cdv(state: ProgramState, operand: u64) -> Result<ProgramState, String> {
        Self::adv(state, operand)
            .map(|state|state.register_a)
            .map(|result|ProgramState { program_counter: state.program_counter + 2, register_c: result, ..state })
    }

    pub fn run_program(&self, program_info: &ProgramInformation) -> Result<Vec<u64>, String> {

        let program = &program_info.program;
        // Check the program sequence length
        if program.len() % 2 != 0 {
            return Err(error::invalid_program_length(program.len()));
        }
        
        let mut state = Self::make_initial_program_state(program_info);
        let mut output = vec![];
        let mut i = 0;
        while state.program_counter + 1 < program.len() as u64 {
            // check if maximum number of steps has been reached
            if i >= self.max_runtime {
                return Err(error::maximum_runtime_reached(self.max_runtime));
            }
            i += 1;

            // do next instruction
            let opcode_idx = state.program_counter as usize;
            let operand_idx = opcode_idx + 1;
            state = match Self::next(state, program[opcode_idx], program[operand_idx], &mut output) {
                Ok(state) => state,
                Err(e) => return Err(e),
            };
        }
        
        return Ok(output)
    }
}

impl Solve<ProgramInformation> for ProgramSimulator {
    fn solve(&self, input: ProgramInformation) -> Result<Answer, String> {
        self.run_program(&input)
            .map(|output|vector_display(&output, ","))
            .map(DisplayableAnswer::new)
    }
}


#[derive(Copy, Clone)]
pub struct ProgramState {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program_counter: u64,
}