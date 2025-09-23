use crate::{Error, Solution};

day!(Day02, 2017, 2, "Corruption Checksum");

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input)
            .iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum::<u32>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input)
            .iter()
            .map(|row| {
                for (i, num1) in row.iter().enumerate() {
                    for (j, num2) in row.iter().enumerate() {
                        if i != j && num1 % num2 == 0 {
                            return num1 / num2;
                        }
                    }
                }
                0
            })
            .sum::<u32>()
            .to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
