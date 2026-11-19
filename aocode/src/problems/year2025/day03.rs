use crate::{Error, Solution};

day!(Day03, 2025, 3, "Lobby");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut total = 0;
        for line in parse_input(input) {
            let mut max = 0;
            for i in 0..line.len() {
                for j in (i + 1)..line.len() {
                    let value = line[i] * 10 + line[j];
                    if value > max {
                        max = value;
                    }
                }
            }
            total += max;
        }
        Ok(total.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        todo!()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u32>> {
    input.lines().map(|line| {
        let line = line.trim();
        let mut result = Vec::with_capacity(line.len());
        line.chars().for_each(|c| {
            if let Some(digit) = c.to_digit(10) {
                result.push(digit);
            }
        });
        result
    })
}
