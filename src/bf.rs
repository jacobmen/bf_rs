/// Enum representing commands found here: https://en.wikipedia.org/wiki/Brainfuck#Commands
#[derive(PartialEq)]
enum Instruction {
    IncrementDP,
    DecrementDP,
    IncrementByte,
    DecrementByte,
    OutputByte,
    ReadByte,
    ConditionalOpen,
    ConditionalClose,
    Ignore,
}

const DATA_SIZE: usize = 30_000;

/// Interpreter for bf based on: https://en.wikipedia.org/wiki/Brainfuck
pub struct BFInterpreter {
    instructions: Vec<Instruction>,
    data: [i8; DATA_SIZE],
    instr_ptr: usize,
    data_ptr: usize,
    // TODO: Add fields for input and output streams to use with '.' and ','
}

impl BFInterpreter {
    // TODO: Return Result after implementing checks for matching parenthesis
    pub fn new(program: &str) -> Self {
        let mut instructions = Vec::with_capacity(program.len());
        for char in program.chars() {
            let instruction = match char {
                '>' => Instruction::IncrementDP,
                '<' => Instruction::DecrementDP,
                '+' => Instruction::IncrementByte,
                '-' => Instruction::DecrementByte,
                '.' => Instruction::OutputByte,
                ',' => Instruction::ReadByte,
                '[' => Instruction::ConditionalOpen,
                ']' => Instruction::ConditionalClose,
                _ => Instruction::Ignore,
            };

            if instruction != Instruction::Ignore {
                instructions.push(instruction);
            }
        }

        Self {
            instructions,
            data: [0; DATA_SIZE],
            instr_ptr: 0,
            data_ptr: 0,
        }
    }
}
