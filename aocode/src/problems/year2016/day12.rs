use crate::{Error, Solution};

day!(Day12, 2016, 12, "Leonardo's Monorail");

impl Solution for Day12 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        cpu.execute();

        Ok(cpu.registers[0].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        cpu.registers[2] = 1; // c = 1
        cpu.execute();

        Ok(cpu.registers[0].to_string())
    }
}

struct Cpu {
    registers: [i64; 4], // a, b, c, d
    instructions: Vec<Instruction>,
}
impl Cpu {
    fn new(input: &str) -> Self {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');
                let instruction = parts.next().unwrap();
                let op1 = parts.next().unwrap();
                let op2 = parts.next().unwrap_or("");

                Instruction::new(instruction, op1, op2)
            })
            .collect();

        Self {
            registers: [0; 4],
            instructions,
        }
    }

    fn execute(&mut self) {
        let mut ins_index = 0;

        while ins_index < self.instructions.len() {
            match &self.instructions[ins_index] {
                Instruction::Cpy(op1, op2) => {
                    let op1_val = self.get_operand_value(op1);
                    match op2 {
                        Operand::Register(reg) => self.registers[*reg] = op1_val,
                        _ => panic!("Invalid operand for cpy instruction"),
                    }

                    ins_index += 1;
                }
                Instruction::Inc(op) => {
                    match op {
                        Operand::Register(reg) => self.registers[*reg] += 1,
                        _ => panic!("Invalid operand for inc instruction"),
                    }

                    ins_index += 1;
                }
                Instruction::Dec(op) => {
                    match op {
                        Operand::Register(reg) => self.registers[*reg] -= 1,
                        _ => panic!("Invalid operand for dec instruction"),
                    }

                    ins_index += 1;
                }
                Instruction::Jnz(op1, op2) => {
                    let offset = self.get_operand_value(op2);

                    let op1_val = self.get_operand_value(op1);

                    if op1_val > 0 {
                        ins_index = (ins_index as i64 + offset) as usize;
                    } else {
                        ins_index += 1;
                    }
                }
            }
        }
    }

    fn get_operand_value(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Register(reg) => self.registers[*reg],
            Operand::Value(value) => *value,
        }
    }
}

enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
}
impl Instruction {
    fn new(op: &str, operand1: &str, operand2: &str) -> Self {
        match op {
            "cpy" => Instruction::Cpy(Operand::new(operand1), Operand::new(operand2)),
            "inc" => Instruction::Inc(Operand::new(operand1)),
            "dec" => Instruction::Dec(Operand::new(operand1)),
            "jnz" => Instruction::Jnz(Operand::new(operand1), Operand::new(operand2)),
            _ => panic!("Unknown instruction: {}", op),
        }
    }
}

enum Operand {
    Register(usize),
    Value(i64),
}
impl Operand {
    fn new(op: &str) -> Self {
        match op.parse::<i64>() {
            Ok(value) => Operand::Value(value),
            Err(_) => Operand::Register(op.chars().next().unwrap() as usize - 'a' as usize),
        }
    }
}
