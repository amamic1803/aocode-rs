use crate::{Error, Solution};

day!(Day05, 2017, 5, "A Maze of Twisty Trampolines, All Alike");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut instructions = parse_input(input);
        let mut index = 0;
        let mut steps = 0;

        while index >= 0 && index < instructions.len() as i32 {
            let jump = instructions[index as usize];
            instructions[index as usize] += 1;
            index += jump;
            steps += 1;
        }

        Ok(steps.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut instructions = parse_input(input);
        let mut index = 0;
        let mut steps = 0;

        while index >= 0 && index < instructions.len() as i32 {
            let jump = instructions[index as usize];
            if jump >= 3 {
                instructions[index as usize] -= 1;
            } else {
                instructions[index as usize] += 1;
            }
            index += jump;
            steps += 1;
        }

        Ok(steps.to_string())
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}
