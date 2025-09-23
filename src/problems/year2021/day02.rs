use crate::{Error, Solution};

day!(Day02, 2021, 2, "Dive!");

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut forward = 0;
        let mut depth = 0;

        for line in input.trim().lines() {
            let mut iter = line.split_whitespace();
            let command_type = iter.next().unwrap();
            let amount: u64 = iter.next().unwrap().parse().unwrap();

            match command_type {
                "forward" => forward += amount,
                "down" => depth += amount,
                "up" => depth = depth.saturating_sub(amount),
                _ => panic!("Invalid command type"),
            }
        }

        Ok((forward * depth).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut forward = 0;
        let mut depth = 0;
        let mut aim = 0;

        for line in input.trim().lines() {
            let mut iter = line.split_whitespace();
            let command_type = iter.next().unwrap();
            let amount: i64 = iter.next().unwrap().parse().unwrap();

            match command_type {
                "forward" => {
                    forward += amount;
                    depth += aim * amount;
                }
                "down" => aim += amount,
                "up" => aim -= amount,
                _ => panic!("Invalid command type"),
            }
        }

        Ok((forward * depth).to_string())
    }
}
