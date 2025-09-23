use crate::{Error, Solution};

day!(Day16, 2017, 16, "Permutation Promenade");

impl Solution for Day16 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let instructions = parse_input(input);
        let mut programs = INITIAL_PROGRAMS;

        dance(&mut programs, &instructions);

        Ok(programs.into_iter().collect())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let instructions = parse_input(input);
        let mut programs = INITIAL_PROGRAMS;
        let mut seen = Vec::new();

        while !seen.contains(&programs) {
            seen.push(programs);
            dance(&mut programs, &instructions);
        }

        let cycle_start = seen.iter().position(|p| p == &programs).unwrap();
        let cycle_len = seen.len() - cycle_start;
        let front_len = cycle_start;

        let mut dances_left = DANCE_ROUNDS - front_len;
        dances_left %= cycle_len;

        for _ in 0..dances_left {
            dance(&mut programs, &instructions);
        }

        Ok(programs.into_iter().collect())
    }
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

const INITIAL_PROGRAMS: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];
const DANCE_ROUNDS: usize = 1_000_000_000;

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for ins in input.trim().split(',') {
        let ins_type = ins.chars().next().unwrap();
        let ins = &ins[1..];
        match ins_type {
            's' => instructions.push(Instruction::Spin(ins.parse().unwrap())),
            'x' => {
                let (a, b) = ins.split_once('/').unwrap();
                let a: usize = a.parse().unwrap();
                let b: usize = b.parse().unwrap();
                instructions.push(Instruction::Exchange(a, b));
            }
            'p' => {
                let (a, b) = ins.split_once('/').unwrap();
                let a = a.chars().next().unwrap();
                let b = b.chars().next().unwrap();
                instructions.push(Instruction::Partner(a, b));
            }
            _ => panic!("Invalid instruction"),
        }
    }
    instructions
}

fn dance(programs: &mut [char], instructions: &[Instruction]) {
    for ins in instructions {
        match ins {
            Instruction::Spin(n) => programs.rotate_right(*n),
            Instruction::Exchange(i, j) => {
                (programs[*i], programs[*j]) = (programs[*j], programs[*i])
            }
            Instruction::Partner(a, b) => {
                let a = programs.iter().position(|&c| c == *a).unwrap();
                let b = programs.iter().position(|&c| c == *b).unwrap();
                (programs[a], programs[b]) = (programs[b], programs[a]);
            }
        }
    }
}
