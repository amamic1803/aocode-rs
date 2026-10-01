use crate::{Error, Solution};

day!(Day08, 2020, 8, "Handheld Halting");

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut console = Console::new(input);
        Ok(console.execute().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut console = Console::new(input);
        Ok(console.execute_fixed().to_string())
    }
}

enum InstructionType {
    Acc,
    Jmp,
    Nop,
}
impl InstructionType {
    fn new(ins_type: &str) -> Self {
        match ins_type {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => panic!("Invalid instruction encountered"),
        }
    }
}
struct Instruction {
    instruction_type: InstructionType,
    value: i32,
    executions: u8,
}
impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut ins_info = instruction.split_whitespace();
        let instruction_type = InstructionType::new(ins_info.next().unwrap());
        let value = ins_info.next().unwrap().parse().unwrap();
        Self {
            instruction_type,
            value,
            executions: 0,
        }
    }
    fn reset(&mut self) {
        self.executions = 0;
    }
}
struct Console {
    instructions: Vec<Instruction>,
    ins_ptr: usize,
    accumulator: i32,
}
impl Console {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::new).collect();
        Self {
            instructions,
            ins_ptr: 0,
            accumulator: 0,
        }
    }
    fn execute(&mut self) -> i32 {
        while self.ins_ptr < self.instructions.len()
            && self.instructions[self.ins_ptr].executions < 1
        {
            self.instructions[self.ins_ptr].executions += 1;
            match self.instructions[self.ins_ptr].instruction_type {
                InstructionType::Acc => {
                    self.accumulator += self.instructions[self.ins_ptr].value;
                    self.ins_ptr += 1;
                }
                InstructionType::Jmp => {
                    self.ins_ptr =
                        (self.ins_ptr as i32 + self.instructions[self.ins_ptr].value) as usize
                }
                InstructionType::Nop => self.ins_ptr += 1,
            }
        }
        self.accumulator
    }
    fn execute_fixed(&mut self) -> i32 {
        for i in 0..self.instructions.len() {
            match self.instructions[i].instruction_type {
                InstructionType::Jmp => {
                    self.instructions[i].instruction_type = InstructionType::Nop
                }
                InstructionType::Nop => {
                    self.instructions[i].instruction_type = InstructionType::Jmp
                }
                _ => continue,
            }
            self.reset();
            let result = self.execute();
            if self.ins_ptr == self.instructions.len() {
                return result;
            }
            match self.instructions[i].instruction_type {
                InstructionType::Jmp => {
                    self.instructions[i].instruction_type = InstructionType::Nop
                }
                InstructionType::Nop => {
                    self.instructions[i].instruction_type = InstructionType::Jmp
                }
                _ => (),
            }
        }
        panic!("No solution found");
    }
    fn reset(&mut self) {
        self.ins_ptr = 0;
        self.accumulator = 0;
        self.instructions.iter_mut().for_each(|f| f.reset());
    }
}
