use crate::{Error, Solution};

day!(Day09, 2023, 9, "Mirage Maintenance");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let data = parse_input(input);
        Ok(data
            .into_iter()
            .map(|row| predict_next(&row))
            .sum::<i64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let data = parse_input(input);
        Ok(data
            .into_iter()
            .map(|row| predict_prev(&row))
            .sum::<i64>()
            .to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn predict_next(data: &[i64]) -> i64 {
    let mut last_values = Vec::new();
    let mut working_values = data.to_vec();

    while working_values.iter().any(|&val| val != 0) {
        last_values.push(working_values[working_values.len() - 1]);

        for i in 0..(working_values.len() - 1) {
            working_values[i] = working_values[i + 1] - working_values[i];
        }

        working_values.pop();
    }

    last_values.into_iter().sum()
}

fn predict_prev(data: &[i64]) -> i64 {
    let mut first_values = Vec::new();
    let mut working_values = data.to_vec();

    while working_values.iter().any(|&val| val != 0) {
        first_values.push(working_values[0]);

        for i in 0..(working_values.len() - 1) {
            working_values[i] = working_values[i + 1] - working_values[i];
        }

        working_values.pop();
    }

    let mut result = 0;
    first_values
        .into_iter()
        .rev()
        .for_each(|val| result = val - result);
    result
}
