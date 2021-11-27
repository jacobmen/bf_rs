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

    /// Runs interpreter on program until termination or forever if it doesn't halt
    pub fn run(&mut self) {
        while !self.terminated {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction = &self.instructions[self.instr_ptr];

        match instruction {
            Instruction::IncrementDP => self.data_ptr += 1,
            Instruction::DecrementDP => self.data_ptr -= 1,
            Instruction::IncrementByte => self.data[self.data_ptr] += 1,
            Instruction::DecrementByte => self.data[self.data_ptr] -= 1,
            Instruction::OutputByte => {
                if let Ok(s) = str::from_utf8(&[self.data[self.data_ptr] as u8]) {
                    print!("{}", s);
                } else {
                    println!(
                        "[ERROR] Couldn't convert byte at index {} to UTF-8",
                        self.data_ptr
                    );
                }
            }
            Instruction::ReadByte => {
                let input: Option<i8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as i8);

                if let Some(byte) = input {
                    self.data[self.data_ptr] = byte;
                } else {
                    eprintln!("[ERROR] Failed to read byte from stdin");
                    return;
                }
            }
            Instruction::ConditionalOpen(close_index) => {
                if self.data[self.data_ptr] == 0 {
                    self.instr_ptr = close_index + 1;
                } else {
                    self.instr_ptr += 1;
                }
            }
            Instruction::ConditionalClose(open_index) => {
                if self.data[self.data_ptr] != 0 {
                    self.instr_ptr = open_index + 1;
                } else {
                    self.instr_ptr += 1;
                }
            }
            Instruction::Ignore => {}
        }

        if !instruction.is_conditional_open() && !instruction.is_conditional_close() {
            self.instr_ptr += 1
        }

        if self.instr_ptr == self.instructions.len() {
            self.terminated = true;
        }
    }
}
