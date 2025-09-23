use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day07, 2015, 7, "Some Assembly Required");

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut circuit = Circuit::new(input);
        circuit.simulate();

        match circuit.get_wire_value("a") {
            Some(a) => Ok(a.to_string()),
            None => Err(Error::NoSolution),
        }
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut circuit = Circuit::new(input);
        let mut circuit2 = circuit.clone();

        circuit.simulate();
        let a = match circuit.get_wire_value("a") {
            Some(a) => a,
            None => panic!("Wire 'a' not found!"),
        };

        circuit2.wires.insert("b", a);
        circuit2.simulate();
        match circuit2.get_wire_value("a") {
            Some(a) => Ok(a.to_string()),
            None => Err(Error::NoSolution),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instruction<'a> {
    Assign(&'a str, &'a str),
    Not(&'a str, &'a str),
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    LShift(&'a str, u16, &'a str),
    RShift(&'a str, u16, &'a str),
}

impl<'a> Instruction<'a> {
    fn new(line: &'a str) -> Self {
        let line_elements: Vec<&str> = line.split(' ').collect();

        if line.contains("NOT") {
            Self::Not(line_elements[1], line_elements[3])
        } else if line.contains("AND") {
            Self::And(line_elements[0], line_elements[2], line_elements[4])
        } else if line.contains("OR") {
            Self::Or(line_elements[0], line_elements[2], line_elements[4])
        } else if line.contains("LSHIFT") {
            Self::LShift(
                line_elements[0],
                line_elements[2]
                    .parse::<u16>()
                    .expect("Invalid left-shift amount!"),
                line_elements[4],
            )
        } else if line.contains("RSHIFT") {
            Self::RShift(
                line_elements[0],
                line_elements[2]
                    .parse::<u16>()
                    .expect("Invalid right-shift amount!"),
                line_elements[4],
            )
        } else {
            Self::Assign(line_elements[0], line_elements[2])
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Circuit<'a> {
    wires: HashMap<&'a str, u16>,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Circuit<'a> {
    fn new(input: &'a str) -> Self {
        let mut instructions = Vec::new();
        for line in input.trim().lines() {
            instructions.push(Instruction::new(line.trim()));
        }

        Self {
            wires: HashMap::new(),
            instructions,
        }
    }

    fn simulate(&mut self) {
        while !self.instructions.is_empty() {
            let mut deleted: usize = 0;
            for i in 0..self.instructions.len() {
                match self.instructions[i - deleted] {
                    Instruction::Assign(a, b) => {
                        if self.wires.contains_key(b) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        match a.parse::<u16>() {
                            Ok(a) => {
                                self.wires.insert(b, a);
                                self.instructions.remove(i - deleted);
                                deleted += 1;
                            }
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires.insert(b, self.wires[a]);
                                    self.instructions.remove(i - deleted);
                                    deleted += 1;
                                }
                            }
                        }
                    }
                    Instruction::Not(a, b) => {
                        if self.wires.contains_key(b) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        match a.parse::<u16>() {
                            Ok(a) => {
                                self.wires.insert(b, !a);
                                self.instructions.remove(i - deleted);
                                deleted += 1;
                            }
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires.insert(b, !self.wires[a]);
                                    self.instructions.remove(i - deleted);
                                    deleted += 1;
                                }
                            }
                        }
                    }
                    Instruction::And(a, b, c) => {
                        if self.wires.contains_key(c) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        let val1 = match a.parse::<u16>() {
                            Ok(a) => a,
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires[a]
                                } else {
                                    continue;
                                }
                            }
                        };
                        let val2 = match b.parse::<u16>() {
                            Ok(b) => b,
                            Err(_) => {
                                if self.wires.contains_key(b) {
                                    self.wires[b]
                                } else {
                                    continue;
                                }
                            }
                        };
                        self.wires.insert(c, val1 & val2);
                        self.instructions.remove(i - deleted);
                        deleted += 1;
                    }
                    Instruction::Or(a, b, c) => {
                        if self.wires.contains_key(c) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        let val1 = match a.parse::<u16>() {
                            Ok(a) => a,
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires[a]
                                } else {
                                    continue;
                                }
                            }
                        };
                        let val2 = match b.parse::<u16>() {
                            Ok(b) => b,
                            Err(_) => {
                                if self.wires.contains_key(b) {
                                    self.wires[b]
                                } else {
                                    continue;
                                }
                            }
                        };
                        self.wires.insert(c, val1 | val2);
                        self.instructions.remove(i - deleted);
                        deleted += 1;
                    }
                    Instruction::LShift(a, b, c) => {
                        if self.wires.contains_key(c) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        match a.parse::<u16>() {
                            Ok(a) => {
                                self.wires.insert(c, a << b);
                                self.instructions.remove(i - deleted);
                                deleted += 1;
                            }
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires.insert(c, self.wires[a] << b);
                                    self.instructions.remove(i - deleted);
                                    deleted += 1;
                                }
                            }
                        }
                    }
                    Instruction::RShift(a, b, c) => {
                        if self.wires.contains_key(c) {
                            self.instructions.remove(i - deleted);
                            deleted += 1;
                            continue;
                        }
                        match a.parse::<u16>() {
                            Ok(a) => {
                                self.wires.insert(c, a >> b);
                                self.instructions.remove(i - deleted);
                                deleted += 1;
                            }
                            Err(_) => {
                                if self.wires.contains_key(a) {
                                    self.wires.insert(c, self.wires[a] >> b);
                                    self.instructions.remove(i - deleted);
                                    deleted += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_wire_value(&self, wire: &str) -> Option<u16> {
        self.wires.get(wire).copied()
    }
}
