use crate::{Error, Solution};

day!(Day06, 2022, 6, "Tuning Trouble");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut characters = 4;
        while !start_of_packet(&input[(characters - 4)..characters]) {
            characters += 1;
        }
        Ok(characters.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut characters = 14;
        while !start_of_packet(&input[(characters - 14)..characters]) {
            characters += 1;
        }
        Ok(characters.to_string())
    }
}

fn start_of_packet(inp: &str) -> bool {
    for x in inp.chars() {
        let mut counted = 0;
        for y in inp.chars() {
            if x == y {
                counted += 1;
            }
        }
        if counted > 1 {
            return false;
        }
    }
    true
}
