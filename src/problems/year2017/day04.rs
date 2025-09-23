use crate::{Error, Solution};

day!(Day04, 2017, 4, "High-Entropy Passphrases");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .trim()
            .lines()
            .filter(|line| {
                let mut words = line.split_whitespace().collect::<Vec<_>>();
                let original_len = words.len();
                words.sort();
                words.dedup();
                original_len == words.len()
            })
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .trim()
            .lines()
            .filter(|line| {
                let mut words = line
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars().collect::<Vec<_>>();
                        chars.sort();
                        chars
                    })
                    .collect::<Vec<_>>();
                let original_len = words.len();
                words.sort();
                words.dedup();
                original_len == words.len()
            })
            .count()
            .to_string())
    }
}
