use crate::{Error, Solution};

day!(Day07, 2024, 7, "Bridge Repair");

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(solve(input, check_equation).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(solve(input, check_equation2).to_string())
    }
}

fn solve<T: Fn(u64, u64, &[u64]) -> bool>(input: &str, check: T) -> u64 {
    let mut calibration_result = 0;

    let mut tmp_values = Vec::new();
    for line in input.lines() {
        let (target, values) = line.split_once(":").unwrap();
        let target = target.parse::<u64>().unwrap();
        tmp_values.clear();
        tmp_values.extend(values.split_whitespace().map(|x| x.parse::<u64>().unwrap()));
        if check(tmp_values[0], target, &tmp_values[1..]) {
            calibration_result += target;
        }
    }

    calibration_result
}
fn check_equation(current: u64, target: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        current == target
    } else if current > target {
        false
    } else {
        let next_plus = current + values[0];
        let next_times = current * values[0];
        let next_values = &values[1..];

        check_equation(next_times, target, next_values)
            || check_equation(next_plus, target, next_values)
    }
}
fn check_equation2(current: u64, target: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        current == target
    } else if current > target {
        false
    } else {
        let next_concat = current * 10u64.pow(values[0].ilog10() + 1) + values[0];
        let next_times = current * values[0];
        let next_plus = current + values[0];
        let next_values = &values[1..];

        check_equation2(next_concat, target, next_values)
            || check_equation2(next_times, target, next_values)
            || check_equation2(next_plus, target, next_values)
    }
}
