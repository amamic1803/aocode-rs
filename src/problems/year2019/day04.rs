use crate::{Error, Solution};
use itertools::Itertools;
use std::collections::HashSet;

day!(Day04, 2019, 4, "Secure Container");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (start, end) = parse_input(input);
        let mut current = start;
        let mut count = 0;

        // Find the first possible password
        first_increasing_pass(&mut current);

        // check all passwords in the given range
        while current <= end {
            // check if the password is valid
            for i in 0..(current.len() - 1) {
                if current[i] == current[i + 1] {
                    count += 1;
                    break;
                }
            }

            // increment password
            if !increment_pass(&mut current) {
                break;
            }
        }

        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (start, end) = parse_input(input);
        let mut current = start;
        let mut count = 0;

        // Find the first possible password
        first_increasing_pass(&mut current);

        // check all passwords in the given range
        let mut double_digit = HashSet::new();
        let mut triple_digit = HashSet::new();
        while current <= end {
            // check if the password is valid
            double_digit.clear();
            for i in 0..(current.len() - 1) {
                if current[i] == current[i + 1] {
                    double_digit.insert(current[i]);
                }
            }
            triple_digit.clear();
            for i in 0..(current.len() - 2) {
                if current[i] == current[i + 1] && current[i] == current[i + 2] {
                    triple_digit.insert(current[i]);
                }
            }
            if double_digit.difference(&triple_digit).count() > 0 {
                count += 1;
            }

            // increment password
            if !increment_pass(&mut current) {
                break;
            }
        }

        Ok(count.to_string())
    }
}

fn parse_input(input: &str) -> ([u8; 6], [u8; 6]) {
    input
        .trim()
        .split('-')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn first_increasing_pass(password: &mut [u8]) {
    for i in 1..password.len() {
        if password[i] < password[i - 1] {
            for j in i..password.len() {
                password[j] = password[i - 1];
            }
            break;
        }
    }
}

fn increment_pass(password: &mut [u8]) -> bool {
    let mut i = password.len() - 1;
    while password[i] == 9 {
        i = match i.checked_sub(1) {
            Some(i) => i,
            None => return false,
        };
    }
    let next_val = password[i] + 1;
    for digit in password.iter_mut().skip(i) {
        *digit = next_val;
    }
    true
}
