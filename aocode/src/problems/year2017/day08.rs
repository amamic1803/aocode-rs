use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day08, 2017, 8, "I Heard You Like Registers");

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        cpu.execute();
        Ok(cpu.largest_register_value().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        cpu.execute();
        Ok(cpu.largest_register_value_ever().to_string())
    }
}

struct Cpu<'a> {
    registers: HashMap<&'a str, i64>,
    instructions: Vec<Instruction<'a>>,
    max_reg_value_ever: i64,
    max_reg_value_now: i64,
}
impl<'a> Cpu<'a> {
    fn new(input: &'a str) -> Self {
        let registers = HashMap::new();
        let instructions = input
            .lines()
            .map(Instruction::new)
            .collect::<Vec<Instruction>>();

        Self {
            registers,
            instructions,
            max_reg_value_ever: 0,
            max_reg_value_now: 0,
        }
    }

    fn execute(&mut self) {
        for instruction in &self.instructions {
            let cmp_reg_val = *self.registers.entry(instruction.cmp_register).or_insert(0);
            let cmp_result = match instruction.cmp_type {
                ">" => cmp_reg_val > instruction.cmp_value,
                "<" => cmp_reg_val < instruction.cmp_value,
                ">=" => cmp_reg_val >= instruction.cmp_value,
                "<=" => cmp_reg_val <= instruction.cmp_value,
                "==" => cmp_reg_val == instruction.cmp_value,
                "!=" => cmp_reg_val != instruction.cmp_value,
                _ => panic!("Invalid comparison type"),
            };
            if cmp_result {
                let reg_val = self.registers.entry(instruction.register).or_insert(0);
                *reg_val += instruction.value;
                if *reg_val > self.max_reg_value_ever {
                    self.max_reg_value_ever = *reg_val;
                }
            }
        }

        self.max_reg_value_now = *self.registers.values().max().unwrap();
    }

    fn largest_register_value(&self) -> i64 {
        self.max_reg_value_now
    }

    fn largest_register_value_ever(&self) -> i64 {
        self.max_reg_value_ever
    }
}
struct Instruction<'a> {
    register: &'a str,
    value: i64,
    cmp_register: &'a str,
    cmp_type: &'a str,
    cmp_value: i64,
}
impl<'a> Instruction<'a> {
    fn new(instruction_str: &'a str) -> Self {
        let instruction_parts = instruction_str.split_whitespace().collect::<Vec<&str>>();
        let register = instruction_parts[0];
        let value = match instruction_parts[1] {
            "inc" => instruction_parts[2].parse::<i64>().unwrap(),
            "dec" => -instruction_parts[2].parse::<i64>().unwrap(),
            _ => panic!("Invalid instruction"),
        };
        let cmp_register = instruction_parts[4];
        let cmp_type = instruction_parts[5];
        let cmp_value = instruction_parts[6].parse::<i64>().unwrap();

        Self {
            register,
            value,
            cmp_register,
            cmp_type,
            cmp_value,
        }
    }
}
