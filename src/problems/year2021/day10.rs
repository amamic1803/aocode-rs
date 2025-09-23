use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day10, 2021, 10, "Syntax Scoring");

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut score_table = HashMap::with_capacity(4);
        score_table.insert(')', 3);
        score_table.insert(']', 57);
        score_table.insert('}', 1197);
        score_table.insert('>', 25137);

        let mut stack = Vec::new();
        let mut error_score = 0;

        for line in input.lines() {
            stack.clear();
            for c in line.chars() {
                if OPEN_BRACES.contains(&c) {
                    stack.push(c);
                } else if CLOSE_BRACES.contains(&c) {
                    if let Some(last) = stack.pop() {
                        if last != OPEN_BRACES[CLOSE_BRACES.iter().position(|&x| x == c).unwrap()] {
                            error_score += score_table.get(&c).unwrap();
                            break;
                        }
                    } else {
                        panic!("Empty stack");
                    }
                } else {
                    panic!("Invalid character: {}", c);
                }
            }
            // if the stack is not clear here, the line is incomplete, and we are ignoring that
        }

        Ok(error_score.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut completion_scores = Vec::new();

        let mut stack = Vec::new();
        'outer: for line in input.lines() {
            stack.clear();

            for c in line.chars() {
                if OPEN_BRACES.contains(&c) {
                    stack.push(c);
                } else if CLOSE_BRACES.contains(&c) {
                    if let Some(last) = stack.pop() {
                        if last != OPEN_BRACES[CLOSE_BRACES.iter().position(|&x| x == c).unwrap()] {
                            continue 'outer; // ignore incomplete lines
                        }
                    } else {
                        panic!("Empty stack");
                    }
                } else {
                    panic!("Invalid character: {}", c);
                }
            }

            if !stack.is_empty() {
                let mut completion_score = 0;
                while let Some(last) = stack.pop() {
                    completion_score = completion_score * 5
                        + OPEN_BRACES.iter().position(|&x| x == last).unwrap() as u64
                        + 1;
                }
                completion_scores.push(completion_score);
            }
        }

        completion_scores.sort();
        Ok(completion_scores[completion_scores.len() / 2].to_string())
    }
}

const OPEN_BRACES: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_BRACES: [char; 4] = [')', ']', '}', '>'];
