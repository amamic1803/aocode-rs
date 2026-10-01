use crate::{Error, Solution};
use pmath::numth::modular::{Congruence, crt};

day!(Day15, 2016, 15, "Timing is Everything");

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let discs = parse_input(input);
        Ok(solve(discs).ok_or(Error::NoSolution)?.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let discs = parse_input(input).chain([(11, 0)]);
        Ok(solve(discs).ok_or(Error::NoSolution)?.to_string())
    }
}

fn solve(discs: impl Iterator<Item = (usize, usize)>) -> Option<i64> {
    crt(discs.enumerate().map(|(i, disc)| {
        let modulus = disc.0 as i64;
        // start + t + i + 1 = 0 (mod modulus)
        let lhs_value = -(disc.1 as i64 + i as i64 + 1);
        Congruence::new(lhs_value, modulus)
    }))
    .map(|solution| solution.0)
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.lines().map(|line| {
        let positions = line.split_whitespace().nth(3).unwrap().parse().unwrap();
        let start = line
            .split_whitespace()
            .nth(11)
            .unwrap()
            .trim_end_matches('.')
            .parse()
            .unwrap();
        (positions, start)
    })
}
