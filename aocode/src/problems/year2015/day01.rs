use crate::{Error, Solution};

day!(Day01, 2015, 1, "Not Quite Lisp");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut floor = 0;
        for c in input.trim().chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }
        Ok(floor.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut floor = 0;
        for (i, c) in input.trim().chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                return Ok((i + 1).to_string());
            }
        }
        Err(Error::NoSolution)
    }
}
