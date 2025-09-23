use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day02, 2018, 2, "Inventory Management System");

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let parsed_input: Vec<HashMap<char, u32>> = input
            .trim()
            .lines()
            .map(|line| {
                let mut hash_map = HashMap::new();
                for charachter in line.chars() {
                    let count = hash_map.entry(charachter).or_insert(0);
                    *count += 1;
                }
                hash_map
            })
            .collect();

        let mut two_count = 0;
        let mut three_count = 0;

        for hash_map in parsed_input {
            let mut two_found = false;
            let mut three_found = false;
            for (_, count) in hash_map.iter() {
                match *count {
                    2 => two_found = true,
                    3 => three_found = true,
                    _ => (),
                }
            }
            if two_found {
                two_count += 1;
            }
            if three_found {
                three_count += 1;
            }
        }

        Ok((two_count * three_count).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut ids = input.trim().lines().collect::<Vec<_>>();
        let different_chars = |input1: &str, input2: &str| {
            let mut count: u32 = 0;
            for (char1, char2) in input1.chars().zip(input2.chars()) {
                if char1 != char2 {
                    count += 1;
                }
            }
            count
        };

        while ids.len() > 2 {
            let id = ids.pop().unwrap();
            for id2 in ids.iter() {
                if different_chars(id, id2) == 1 {
                    return Ok(id
                        .chars()
                        .zip(id2.chars())
                        .filter(|(char1, char2)| char1 == char2)
                        .map(|(char1, _)| char1)
                        .collect());
                }
            }
        }

        if different_chars(ids[0], ids[1]) == 1 {
            Ok(ids[0]
                .chars()
                .zip(ids[1].chars())
                .filter(|(char1, char2)| char1 == char2)
                .map(|(char1, _)| char1)
                .collect())
        } else {
            Err(Error::NoSolution)
        }
    }
}
