use crate::{Error, Solution};
use pmath::primes::is_prime;

day!(Day23, 2017, 23, "Coprocessor Conflagration");

impl Solution for Day23 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        Ok(cpu.simulate().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        // after analyzing the instructions in the input, it is clear that the program
        // is counting the number of composite numbers between b and c with some step.
        // b is initialized to x * y + z
        // c is initialized to b + w
        // x is found in the 1st instruction
        // y is found in the 5th instruction
        // -z is found in the 6th instruction
        // -w is found in the 8th instruction
        // -step is found in the 31st instruction

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut w = 0;
        let mut step = 0;

        for (i, ins) in input.trim().lines().enumerate() {
            match i + 1 {
                1 => {
                    x = ins
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                }
                5 => {
                    y = ins
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                }
                6 => {
                    z = -ins
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                }
                8 => {
                    w = -ins
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                }
                31 => {
                    step = -ins
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                }
                _ => {}
            }
        }

        let b = x * y + z;
        Ok(((b as u64)..=((b + w) as u64))
            .step_by(step as usize)
            .filter(|&i| !is_prime(i).0)
            .count()
            .to_string())
    }
}

struct Cpu<'a> {
    registers: [i64; 8],
    instructions: Vec<Instruction<'a>>,
}
impl<'a> Cpu<'a> {
    fn new(instructions: &'a str) -> Self {
        let registers = [0; 8];

        let instructions = instructions
            .trim()
            .lines()
            .map(Instruction::new)
            .collect::<Vec<_>>();

        Self {
            registers,
            instructions,
        }
    }

    fn get_reg(&self, r: char) -> i64 {
        self.registers[r as usize - 'a' as usize]
    }

    fn get_reg_mut(&mut self, r: char) -> &mut i64 {
        &mut self.registers[r as usize - 'a' as usize]
    }

    fn simulate(&mut self) -> u32 {
        let mut mul_count = 0;
        let mut ins_ptr = 0;

        while ins_ptr < self.instructions.len() {
            match self.instructions[ins_ptr].op {
                "set" => {
                    let y_val = match self.instructions[ins_ptr].arg2 {
                        Operand::Register(r) => self.get_reg(r),
                        Operand::Value(v) => v,
                    };
                    if let Operand::Register(r) = self.instructions[ins_ptr].arg1 {
                        *self.get_reg_mut(r) = y_val;
                    }
                    ins_ptr += 1;
                }
                "sub" => {
                    let y_val = match self.instructions[ins_ptr].arg2 {
                        Operand::Register(r) => self.get_reg(r),
                        Operand::Value(v) => v,
                    };
                    if let Operand::Register(r) = self.instructions[ins_ptr].arg1 {
                        *self.get_reg_mut(r) -= y_val;
                    }
                    ins_ptr += 1;
                }
                "mul" => {
                    let y_val = match self.instructions[ins_ptr].arg2 {
                        Operand::Register(r) => self.get_reg(r),
                        Operand::Value(v) => v,
                    };
                    if let Operand::Register(r) = self.instructions[ins_ptr].arg1 {
                        *self.get_reg_mut(r) *= y_val;
                    }
                    ins_ptr += 1;
                    mul_count += 1;
                }
                "jnz" => {
                    let x_val = match self.instructions[ins_ptr].arg1 {
                        Operand::Register(r) => self.get_reg(r),
                        Operand::Value(v) => v,
                    };
                    if x_val != 0 {
                        let y_val = match self.instructions[ins_ptr].arg2 {
                            Operand::Register(r) => self.get_reg(r),
                            Operand::Value(v) => v,
                        };
                        let temp_ptr = ins_ptr as i64 + y_val;
                        if temp_ptr < 0 {
                            break;
                        } else {
                            ins_ptr = temp_ptr as usize;
                        }
                    } else {
                        ins_ptr += 1;
                    }
                }
                _ => panic!("Unknown instruction"),
            }
        }

        mul_count
    }
}
struct Instruction<'a> {
    op: &'a str,
    arg1: Operand,
    arg2: Operand,
}
impl<'a> Instruction<'a> {
    fn new(ins: &'a str) -> Self {
        let mut split = ins.split_whitespace();
        let op = split.next().unwrap();
        let arg1 = Operand::new(split.next().unwrap());
        let arg2 = Operand::new(split.next().unwrap());
        Self { op, arg1, arg2 }
    }
}
enum Operand {
    Register(char),
    Value(i64),
}
impl Operand {
    fn new(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            Operand::Value(i)
        } else {
            Operand::Register(s.chars().next().unwrap())
        }
    }
}
