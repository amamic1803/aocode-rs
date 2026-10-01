use crate::{Error, Solution};

day!(Day25, 2016, 25, "Clock Signal");

impl Solution for Day25 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        // input code is meant to be analyzed by hand
        // what code does is take a value, add some constant to it (found in the input),
        // and output its binary digits starting from the least significant bit
        // so to get alternating 0 and 1,
        // the number must be in the form 1010...1010

        // since the value is a positive integer,
        // we need to find the first number of the form 1010...1010
        // greater than the constant and subtract the constant from it to get the solution

        let constant = input
            .lines()
            .skip(1)
            .take(2)
            .map(|line| {
                line.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .product::<usize>();

        let mut number = 0;
        while number < constant {
            number <<= 2;
            number |= 0b10;
        }

        Ok((number - constant).to_string())
    }

    fn part2(&self, _input: &str) -> Result<String, Error> {
        Ok(String::from("Advent of Code 2016 solved!"))
    }
}
