use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day06, 2016, 6, "Signals and Noise");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(solve(input, false))
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(solve(input, true))
    }
}

fn solve(input: &str, minimum: bool) -> String {
    let message_len = input.trim().lines().next().unwrap().trim().chars().count();
    let mut message: Vec<HashMap<char, usize>> = Vec::with_capacity(message_len);
    for _ in 0..message_len {
        message.push(HashMap::new());
    }

    for line in input.trim().lines() {
        for (i, c) in line.chars().enumerate() {
            let count = message[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut result = String::new();

    for val in message {
        result.push(if minimum {
            *val.iter().min_by_key(|&(_, v)| v).unwrap().0
        } else {
            *val.iter().max_by_key(|&(_, v)| v).unwrap().0
        });
    }

    result
}
