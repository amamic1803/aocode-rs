use crate::{Error, Solution};
use md5::{Digest, Md5};
use std::fmt::Write;

day!(Day04, 2015, 4, "The Ideal Stocking Stuffer");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().to_string();
        let input_len = input.len();
        let mut hasher = Md5::new();

        for i in 0.. {
            input.truncate(input_len);
            write!(&mut input, "{}", i).unwrap();
            hasher.update(&input);
            let hash = hasher.finalize_reset();
            if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
                return Ok(i.to_string());
            }
        }

        unreachable!()
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().to_string();
        let input_len = input.len();
        let mut hasher = Md5::new();

        for i in 0.. {
            input.truncate(input_len);
            write!(&mut input, "{}", i).unwrap();
            hasher.update(&input);
            let hash = hasher.finalize_reset();
            if hash.starts_with(&[0, 0, 0]) {
                return Ok(i.to_string());
            }
        }

        unreachable!()
    }
}
