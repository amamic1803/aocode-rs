use crate::{Error, Solution};

day!(Day15, 2017, 15, "Dueling Generators");

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (mut gen_a, mut gen_b) = parse_input(input);
        let mut matches = 0;

        for _ in 0..ITERATIONS_1 {
            gen_a = (gen_a * GEN_A_FACT) % MOD;
            gen_b = (gen_b * GEN_B_FACT) % MOD;

            if gen_a as u16 == gen_b as u16 {
                matches += 1;
            }
        }

        Ok(matches.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (mut gen_a, mut gen_b) = parse_input(input);
        let mut matches = 0;

        for _ in 0..ITERATIONS_2 {
            gen_a = (gen_a * GEN_A_FACT) % MOD;
            while gen_a % GEN_A_MULT != 0 {
                gen_a = (gen_a * GEN_A_FACT) % MOD;
            }
            gen_b = (gen_b * GEN_B_FACT) % MOD;
            while gen_b % GEN_B_MULT != 0 {
                gen_b = (gen_b * GEN_B_FACT) % MOD;
            }

            if gen_a as u16 == gen_b as u16 {
                matches += 1;
            }
        }

        Ok(matches.to_string())
    }
}

const GEN_A_FACT: u64 = 16807;
const GEN_B_FACT: u64 = 48271;
const MOD: u64 = 2147483647;
const ITERATIONS_1: u32 = 40_000_000;
const ITERATIONS_2: u32 = 5_000_000;
const GEN_A_MULT: u64 = 4;
const GEN_B_MULT: u64 = 8;

fn parse_input(input: &str) -> (u64, u64) {
    let mut a = 0;
    let mut b = 0;

    for line in input.trim().lines() {
        if line.contains('A') {
            a = line.split_whitespace().last().unwrap().parse().unwrap();
        } else if line.contains('B') {
            b = line.split_whitespace().last().unwrap().parse().unwrap();
        } else {
            panic!("Invalid input");
        }
    }

    (a, b)
}
