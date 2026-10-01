use crate::{Error, Solution};

day!(Day01, 2021, 1, "Sonar Sweep");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut prev = usize::MAX;
        let mut count = 0;

        for line in input.trim().lines() {
            let num = line.parse::<usize>().unwrap();
            if num > prev {
                count += 1;
            }
            prev = num;
        }

        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let input: Vec<usize> = input
            .trim()
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();
        let mut count = 0;

        for i in 0..(input.len() - 3) {
            if (input[i + 1] + input[i + 2] + input[i + 3])
                > (input[i] + input[i + 1] + input[i + 2])
            {
                count += 1;
            }
        }

        Ok(count.to_string())
    }
}
