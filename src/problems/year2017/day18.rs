use crate::{Error, Solution};
use std::collections::{HashMap, VecDeque};

day!(Day18, 2017, 18, "Duet");

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut cpu = Cpu::new(input);
        Ok(cpu.simulate().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut multi_cpu = MultiCpu::new(input);
        Ok(multi_cpu.simulate().to_string())
    }
}

struct Cpu {
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
}
impl Cpu {
    fn new(instructions: &str) -> Self {
        let registers = HashMap::new();

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

    fn simulate(&mut self) -> i64 {
        let mut ins_ptr = 0;
        let mut last_freq = 0;

        while ins_ptr < self.instructions.len() {
            match &self.instructions[ins_ptr] {
                Instruction::Snd(oper1) => {
                    last_freq = match oper1 {
                        Operand::Value(i) => *i,
                        Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                    };
                    ins_ptr += 1;
                }
                Instruction::Set(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                        };
                        self.registers.insert(*r, val_y);
                    }
                    ins_ptr += 1;
                }
                Instruction::Add(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                        };
                        *self.registers.entry(*r).or_insert(0) += val_y;
                    }
                    ins_ptr += 1;
                }
                Instruction::Mul(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                        };
                        *self.registers.entry(*r).or_insert(0) *= val_y;
                    }
                    ins_ptr += 1;
                }
                Instruction::Mod(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                        };
                        *self.registers.entry(*r).or_insert(0) %= val_y;
                    }
                    ins_ptr += 1;
                }
                Instruction::Rcv(oper1) => {
                    let val = match oper1 {
                        Operand::Value(i) => *i,
                        Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                    };
                    if val != 0 {
                        break;
                    }
                    ins_ptr += 1;
                }
                Instruction::Jgz(oper1, oper2) => {
                    let val_x = match oper1 {
                        Operand::Value(i) => *i,
                        Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                    };
                    if val_x > 0 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers.entry(*r).or_insert(0),
                        };
                        let temp_ptr = ins_ptr as i64 + val_y;
                        if temp_ptr < 0 {
                            break;
                        } else {
                            ins_ptr = temp_ptr as usize;
                        }
                    } else {
                        ins_ptr += 1;
                    }
                }
            }
        }

        last_freq
    }
}
struct MultiCpu {
    registers: [HashMap<char, i64>; 2],
    ins_ptrs: [usize; 2],
    queues: [VecDeque<i64>; 2],
    instructions: Vec<Instruction>,
}
impl MultiCpu {
    fn new(instructions: &str) -> Self {
        let mut registers = [HashMap::new(), HashMap::new()];
        registers[0].insert('p', 0);
        registers[1].insert('p', 1);
        let ins_ptrs = [0, 0];
        let queues = [VecDeque::new(), VecDeque::new()];

        let instructions = instructions
            .trim()
            .lines()
            .map(Instruction::new)
            .collect::<Vec<_>>();

        Self {
            registers,
            ins_ptrs,
            queues,
            instructions,
        }
    }

    fn simulate(&mut self) -> u32 {
        let mut prog1_sends = 0;
        let mut turn = 0;

        // in the start, queues are empty, but the programs are not waiting, so loop shouldn't end
        let mut prog_waiting = [false; 2];

        loop {
            if prog_waiting[turn]
                && self.queues[turn].is_empty()
                && self.queues[1 - turn].is_empty()
            {
                break;
            }

            match &self.instructions[self.ins_ptrs[turn]] {
                Instruction::Snd(oper1) => {
                    let val = match oper1 {
                        Operand::Value(i) => *i,
                        Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                    };
                    self.queues[1 - turn].push_back(val);
                    if turn == 1 {
                        prog1_sends += 1;
                    }
                    self.ins_ptrs[turn] += 1;
                }
                Instruction::Set(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                        };
                        self.registers[turn].insert(*r, val_y);
                    }
                    self.ins_ptrs[turn] += 1;
                }
                Instruction::Add(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                        };
                        *self.registers[turn].entry(*r).or_insert(0) += val_y;
                    }
                    self.ins_ptrs[turn] += 1;
                }
                Instruction::Mul(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                        };
                        *self.registers[turn].entry(*r).or_insert(0) *= val_y;
                    }
                    self.ins_ptrs[turn] += 1;
                }
                Instruction::Mod(oper1, oper2) => {
                    if let Operand::Register(r) = oper1 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                        };
                        *self.registers[turn].entry(*r).or_insert(0) %= val_y;
                    }
                    self.ins_ptrs[turn] += 1;
                }
                Instruction::Rcv(oper1) => {
                    if let Some(val) = self.queues[turn].pop_front() {
                        if let Operand::Register(r) = oper1 {
                            self.registers[turn].insert(*r, val);
                        } else {
                            panic!("Rcv instruction with value operand");
                        }
                        self.ins_ptrs[turn] += 1;
                    } else {
                        prog_waiting[turn] = true;
                        turn = 1 - turn;
                    }
                }
                Instruction::Jgz(oper1, oper2) => {
                    let val_x = match oper1 {
                        Operand::Value(i) => *i,
                        Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                    };
                    if val_x > 0 {
                        let val_y = match oper2 {
                            Operand::Value(i) => *i,
                            Operand::Register(r) => *self.registers[turn].entry(*r).or_insert(0),
                        };
                        let temp_ptr = self.ins_ptrs[turn] as i64 + val_y;
                        if temp_ptr < 0 {
                            break;
                        } else {
                            self.ins_ptrs[turn] = temp_ptr as usize;
                        }
                    } else {
                        self.ins_ptrs[turn] += 1;
                    }
                }
            }
        }

        prog1_sends
    }
}
enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}
impl Instruction {
    fn new(ins: &str) -> Self {
        let mut split = ins.split_whitespace();

        let op = split.next().unwrap();
        let arg1 = Operand::new(split.next().unwrap());

        match op {
            "snd" => Self::Snd(arg1),
            "set" => {
                let arg2 = Operand::new(split.next().unwrap());
                Self::Set(arg1, arg2)
            }
            "add" => {
                let arg2 = Operand::new(split.next().unwrap());
                Self::Add(arg1, arg2)
            }
            "mul" => {
                let arg2 = Operand::new(split.next().unwrap());
                Self::Mul(arg1, arg2)
            }
            "mod" => {
                let arg2 = Operand::new(split.next().unwrap());
                Self::Mod(arg1, arg2)
            }
            "rcv" => Self::Rcv(arg1),
            "jgz" => {
                let arg2 = Operand::new(split.next().unwrap());
                Self::Jgz(arg1, arg2)
            }
            _ => panic!("Unknown instruction"),
        }
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
