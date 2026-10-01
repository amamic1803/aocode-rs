use crate::{Error, Solution};
use std::fmt::Write;

day!(Day10, 2017, 10, "Knot Hash");

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let lengths = input
            .trim()
            .split(',')
            .map(|num| num.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let mut knot_hash = KnotHash::new();

        knot_hash.round(&lengths);

        Ok((knot_hash.list[0] as u16 * knot_hash.list[1] as u16).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut knot_hash = KnotHash::new();
        Ok(knot_hash.hash(input.trim()))
    }
}

const LIST_SIZE: usize = 256;
const ASCII_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
const ROUNDS: usize = 64;

pub(crate) struct KnotHash {
    list: [u8; LIST_SIZE],
    current_position: usize,
    skip_size: usize,
}
impl KnotHash {
    pub(crate) fn new() -> Self {
        let mut list = [0; LIST_SIZE];
        for (i, item) in list.iter_mut().enumerate() {
            *item = i as u8;
        }
        Self {
            list,
            current_position: 0,
            skip_size: 0,
        }
    }

    fn execute_step(&mut self, len: u8) {
        // reverse order
        let mut start = self.current_position;
        let mut end = start + len as usize - 1; // inclusive
        while start < end {
            self.list.swap(start % LIST_SIZE, end % LIST_SIZE);
            start += 1;
            end -= 1;
        }

        // move current position
        self.current_position = (self.current_position + len as usize + self.skip_size) % LIST_SIZE;

        // increment skip size
        self.skip_size += 1;
    }

    fn round(&mut self, lengths: &[u8]) {
        for len in lengths {
            self.execute_step(*len);
        }
    }

    pub(crate) fn hash(&mut self, input: &str) -> String {
        // convert input to lengths
        let lengths = input
            .chars()
            .map(|c| c as u8)
            .chain(ASCII_SUFFIX.iter().copied())
            .collect::<Vec<_>>();

        // run rounds
        for _ in 0..ROUNDS {
            self.round(&lengths);
        }

        // condense data
        let mut condensed = [0; LIST_SIZE / 16];
        for (i, item) in condensed.iter_mut().enumerate() {
            let start = i * 16;
            let end = start + 16;
            *item = self.list[start..end].iter().fold(0, |acc, &x| acc ^ x);
        }

        // format as hex
        let mut result_str = String::with_capacity(condensed.len() * 2);
        for byte in &condensed {
            write!(result_str, "{:02x}", byte).unwrap();
        }
        result_str
    }
}
