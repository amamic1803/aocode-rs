use crate::{Error, Solution};

day!(Day23, 2016, 23, "Safe Cracking");

impl Solution for Day23 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Keypad::new(input).execute(INPUT1).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        // after analysing the input, there is obviously one multiplication loop that runs many times
        // I optimized it manually and then wrote a method to find it in the input
        // after the loop is found, it is optimized to multiplication
        // the loop looks like this:
        // cpy b c
        // inc a
        // dec c
        // jnz c -2
        // dec d
        // jnz d -5
        // where
        //      a is the destination register
        //      b is the multiplier register
        //      c is the addend register
        //      d is the temporary register
        // after the loop, the multiplier and temporary registers are cleared,
        // addend register is unchanged, and destination register is set to the multiplier * (addend + destination)
        // the search for loop is repeated each time tgl instruction actually changes something
        Ok(Keypad::new(input).execute(INPUT2).to_string())
    }
}

const INPUT1: i64 = 7;
const INPUT2: i64 = 12;

struct Keypad {
    registers: [i64; 4], // a, b, c, d
    instructions: Vec<Instruction>,
    mul_loop: Loop,
}
impl Keypad {
    fn new(input: &str) -> Self {
        let registers = [0; 4];
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
        let mul_loop = Loop {
            start: 0,
            end: 0,
            destination: 0,
            multiplier: 0,
            addend: 0,
            clear: 0,
        };

        let mut keypad = Self {
            registers,
            instructions,
            mul_loop,
        };
        keypad.calculate_loop();
        keypad
    }

    fn execute(&mut self, n: i64) -> i64 {
        self.registers[0] = n;
        let mut ins_ptr = 0;

        while ins_ptr < self.instructions.len() {
            if ins_ptr == self.mul_loop.start {
                self.registers[self.mul_loop.destination] = self.registers
                    [self.mul_loop.multiplier]
                    * (self.registers[self.mul_loop.addend]
                        + self.registers[self.mul_loop.destination]);
                self.registers[self.mul_loop.multiplier] = 0;
                self.registers[self.mul_loop.clear] = 0;
                ins_ptr = self.mul_loop.end;
                continue;
            }

            match &self.instructions[ins_ptr] {
                Instruction::Cpy(op1, op2) => {
                    if let Operand::Register(reg) = op2 {
                        self.registers[*reg] = self.get_operand_value(op1);
                    }
                    ins_ptr += 1;
                }
                Instruction::Inc(op) => {
                    if let Operand::Register(reg) = op {
                        self.registers[*reg] += 1;
                    }
                    ins_ptr += 1;
                }
                Instruction::Dec(op) => {
                    if let Operand::Register(reg) = op {
                        self.registers[*reg] -= 1;
                    }
                    ins_ptr += 1;
                }
                Instruction::Jnz(op1, op2) => {
                    let cond_val = self.get_operand_value(op1);
                    let offset = self.get_operand_value(op2);

                    if cond_val > 0 {
                        ins_ptr = (ins_ptr as i64 + offset) as usize;
                    } else {
                        ins_ptr += 1;
                    }
                }
                Instruction::Tgl(op) => {
                    let offset = self.get_operand_value(op);
                    let target = ins_ptr as i64 + offset;

                    if target >= 0 && target < self.instructions.len() as i64 {
                        let new_ins = match self.instructions[target as usize] {
                            Instruction::Cpy(op1, op2) => Instruction::Jnz(op1, op2),
                            Instruction::Inc(op) => Instruction::Dec(op),
                            Instruction::Dec(op) => Instruction::Inc(op),
                            Instruction::Jnz(op1, op2) => Instruction::Cpy(op1, op2),
                            Instruction::Tgl(op) => Instruction::Inc(op),
                        };

                        self.instructions[target as usize] = new_ins;
                        self.calculate_loop();
                    }

                    ins_ptr += 1;
                }
            }
        }

        self.registers[0]
    }

    fn get_operand_value(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Register(reg) => self.registers[*reg],
            Operand::Value(value) => *value,
        }
    }

    fn calculate_loop(&mut self) {
        for i in 0..(self.instructions.len() - 5) {
            if let Instruction::Cpy(Operand::Register(addend), Operand::Register(clear)) =
                self.instructions[i]
                && let Instruction::Inc(Operand::Register(destination)) = self.instructions[i + 1]
                && let Instruction::Dec(Operand::Register(clr)) = self.instructions[i + 2]
            {
                if clr != clear {
                    continue;
                }
                if let Instruction::Jnz(Operand::Register(clr), Operand::Value(-2)) =
                    self.instructions[i + 3]
                {
                    if clr != clear {
                        continue;
                    }
                    if let Instruction::Dec(Operand::Register(multiplier)) =
                        self.instructions[i + 4]
                        && let Instruction::Jnz(Operand::Register(mul), Operand::Value(-5)) =
                            self.instructions[i + 5]
                    {
                        if mul != multiplier {
                            continue;
                        }
                        self.mul_loop = Loop {
                            start: i,
                            end: i + 6,
                            destination,
                            multiplier,
                            addend,
                            clear,
                        };
                        break;
                    }
                }
            }
        }
    }
}
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
}
impl Instruction {
    fn new(op: &str, operand1: &str, operand2: &str) -> Self {
        match op {
            "cpy" => Self::Cpy(Operand::new(operand1), Operand::new(operand2)),
            "inc" => Self::Inc(Operand::new(operand1)),
            "dec" => Self::Dec(Operand::new(operand1)),
            "jnz" => Self::Jnz(Operand::new(operand1), Operand::new(operand2)),
            "tgl" => Self::Tgl(Operand::new(operand1)),
            _ => panic!("Unknown instruction: {}", op),
        }
    }
}
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
struct Loop {
    start: usize,
    end: usize,
    destination: usize,
    multiplier: usize,
    addend: usize,
    clear: usize,
}
