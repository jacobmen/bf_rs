use std::io::Read;
use std::str;

/// Enum representing commands found here: https://en.wikipedia.org/wiki/Brainfuck#Commands
#[derive(PartialEq)]
enum Instruction {
    IncrementDP,
    DecrementDP,
    IncrementByte,
    DecrementByte,
    OutputByte,
    ReadByte,
    ConditionalOpen(usize),
    ConditionalClose(usize),
    Ignore,
}

impl Instruction {
    /// Returns `true` if the instruction is [`ConditionalOpen`].
    ///
    /// [`ConditionalOpen`]: Instruction::ConditionalOpen
    fn is_conditional_open(&self) -> bool {
        matches!(self, Self::ConditionalOpen(..))
    }

    /// Returns `true` if the instruction is [`ConditionalClose`].
    ///
    /// [`ConditionalClose`]: Instruction::ConditionalClose
    fn is_conditional_close(&self) -> bool {
        matches!(self, Self::ConditionalClose(..))
    }
}

const DATA_SIZE: usize = 30_000;

/// Interpreter for bf based on: https://en.wikipedia.org/wiki/Brainfuck
pub struct Interpreter {
    instructions: Vec<Instruction>,
    data: [i8; DATA_SIZE],
    instr_ptr: usize,
    data_ptr: usize,
    terminated: bool,
}

impl Interpreter {
    // TODO: Return Result after implementing checks for matching parenthesis
    pub fn new(program: &str) -> Self {
        let mut instructions = Vec::with_capacity(program.len());
        let mut matching_pair_indices = Vec::new();

        for char in program.chars() {
            let instruction = match char {
                '>' => Instruction::IncrementDP,
                '<' => Instruction::DecrementDP,
                '+' => Instruction::IncrementByte,
                '-' => Instruction::DecrementByte,
                '.' => Instruction::OutputByte,
                ',' => Instruction::ReadByte,
                '[' => {
                    matching_pair_indices.push(instructions.len());
                    Instruction::ConditionalOpen(0)
                }
                ']' => {
                    let opening_pair = matching_pair_indices.pop().expect("shouldn't be empty");
                    instructions[opening_pair] = Instruction::ConditionalOpen(instructions.len());
                    Instruction::ConditionalClose(opening_pair)
                }
                _ => Instruction::Ignore,
            };

            if instruction != Instruction::Ignore {
                instructions.push(instruction);
            }
        }

        assert!(matching_pair_indices.is_empty());

        Self {
            instructions,
            data: [0; DATA_SIZE],
            instr_ptr: 0,
            data_ptr: 0,
            terminated: false,
        }
    }

        }
    }
}
