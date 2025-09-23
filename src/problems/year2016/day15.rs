use crate::math::chinese_remainder_theorem;
use crate::{Error, Solution};
use std::cmp::Reverse;

day!(Day15, 2016, 15, "Timing is Everything");

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let discs = parse_input(input);
        Ok(solve(discs).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let discs = parse_input(input).chain([(11, 0)]);
        Ok(solve(discs).to_string())
    }
}

fn solve(discs: impl Iterator<Item = (usize, usize)>) -> u64 {
    let mut congruences = Vec::new();
    for (i, disc) in discs.enumerate() {
        let rhs_value = (-(disc.1 as i64 + i as i64 + 1)).rem_euclid(disc.0 as i64);
        congruences.push((rhs_value as u64, disc.0 as u64));
    }

    congruences.sort_by_key(|&(_, modulus)| Reverse(modulus));
    chinese_remainder_theorem(&congruences)
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
