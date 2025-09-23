use crate::{Error, Solution};

day!(Day21, 2016, 21, "Scrambled Letters and Hash");

impl Solution for Day21 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let computer = PasswordComputer::new(input);
        Ok(computer.scramble(PASSWORD))
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let computer = PasswordComputer::new(input);
        Ok(computer.unscramble(SCRAMBLED_PASSWORD))
    }
}

const PASSWORD: &str = "abcdefgh";
const SCRAMBLED_PASSWORD: &str = "fbgdceah";

enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}
impl Instruction {
    fn from_str(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().collect();
        match words[0] {
            "swap" => match words[1] {
                "position" => {
                    Instruction::SwapPosition(words[2].parse().unwrap(), words[5].parse().unwrap())
                }
                "letter" => Instruction::SwapLetter(
                    words[2].chars().next().unwrap(),
                    words[5].chars().next().unwrap(),
                ),
                _ => panic!("Invalid swap instruction"),
            },
            "rotate" => match words[1] {
                "left" => Instruction::RotateLeft(words[2].parse().unwrap()),
                "right" => Instruction::RotateRight(words[2].parse().unwrap()),
                "based" => Instruction::RotateBasedOnLetter(words[6].chars().next().unwrap()),
                _ => panic!("Invalid rotate instruction"),
            },
            "reverse" => Instruction::Reverse(words[2].parse().unwrap(), words[4].parse().unwrap()),
            "move" => Instruction::Move(words[2].parse().unwrap(), words[5].parse().unwrap()),
            _ => panic!("Invalid instruction"),
        }
    }
}
struct PasswordComputer {
    instructions: Vec<Instruction>,
}
impl PasswordComputer {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::from_str).collect();
        Self { instructions }
    }

    fn scramble(&self, password: &str) -> String {
        let mut password = password.chars().collect::<Vec<char>>();

        for instruction in &self.instructions {
            match *instruction {
                Instruction::SwapPosition(x, y) => password.swap(x, y),
                Instruction::SwapLetter(x, y) => {
                    let pos1 = password.iter().position(|&c| c == x).unwrap();
                    let pos2 = password.iter().position(|&c| c == y).unwrap();
                    password.swap(pos1, pos2);
                }
                Instruction::RotateLeft(x) => password.rotate_left(x),
                Instruction::RotateRight(x) => password.rotate_right(x),
                Instruction::RotateBasedOnLetter(x) => {
                    let pos = password.iter().position(|&c| c == x).unwrap();
                    let mut amount = pos + 1;
                    if pos >= 4 {
                        amount += 1;
                    }
                    amount %= password.len();
                    password.rotate_right(amount);
                }
                Instruction::Reverse(x, y) => password[x..=y].reverse(),
                Instruction::Move(x, y) => {
                    let removed_letter = password.remove(x);
                    password.insert(y, removed_letter);
                }
            }
        }

        password.into_iter().collect()
    }

    fn unscramble(&self, password: &str) -> String {
        let mut password = password.chars().collect::<Vec<char>>();

        for instruction in self.instructions.iter().rev() {
            match *instruction {
                Instruction::SwapPosition(x, y) => password.swap(x, y),
                Instruction::SwapLetter(x, y) => {
                    let pos1 = password.iter().position(|&c| c == x).unwrap();
                    let pos2 = password.iter().position(|&c| c == y).unwrap();
                    password.swap(pos1, pos2);
                }
                Instruction::RotateLeft(x) => password.rotate_right(x),
                Instruction::RotateRight(x) => password.rotate_left(x),
                Instruction::RotateBasedOnLetter(x) => {
                    let mut correct_parent_rotations = Vec::with_capacity(password.len());
                    let all_parent_rotations = (0..password.len()).map(|i| {
                        let mut password = password.clone();
                        password.rotate_left(i);
                        password
                    });
                    for parent_rotation in all_parent_rotations {
                        let mut rotation = parent_rotation.clone();
                        let pos = rotation.iter().position(|&c| c == x).unwrap();
                        let mut amount = pos + 1;
                        if pos >= 4 {
                            amount += 1;
                        }
                        amount %= rotation.len();
                        rotation.rotate_right(amount);
                        if rotation == password {
                            correct_parent_rotations.push(parent_rotation);
                        }
                    }
                    assert_eq!(
                        correct_parent_rotations.len(),
                        1,
                        "Multiple parent rotations found, cannot unscramble!"
                    );
                    password = correct_parent_rotations.into_iter().next().unwrap();
                }
                Instruction::Reverse(x, y) => password[x..=y].reverse(),
                Instruction::Move(x, y) => {
                    let removed_letter = password.remove(y);
                    password.insert(x, removed_letter);
                }
            }
        }

        password.into_iter().collect()
    }
}
