use crate::math::{boundary_points, interior_points};
use crate::{Error, Solution};
use itertools::Itertools;

day!(Day18, 2023, 18, "Lavaduct Lagoon");

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let points = parse_input1(input);
        Ok(calculate_volume(&points).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let points = parse_input2(input);
        Ok(calculate_volume(&points).to_string())
    }
}

fn calculate_volume(points: &[(i64, i64)]) -> i64 {
    let boundary_points = boundary_points(points);
    boundary_points + interior_points(points, boundary_points)
}

fn parse_input1(input: &str) -> Vec<(i64, i64)> {
    let mut points = Vec::new();

    let mut curr_point = (0, 0);
    for line in input.trim().lines() {
        let (ins, val, _) = line.split_whitespace().collect_tuple().unwrap();
        let val = val.parse::<i64>().unwrap();

        curr_point = match ins {
            "R" => (curr_point.0 + val, curr_point.1),
            "L" => (curr_point.0 - val, curr_point.1),
            "U" => (curr_point.0, curr_point.1 + val),
            "D" => (curr_point.0, curr_point.1 - val),
            _ => panic!("Invalid instruction"),
        };

        points.push(curr_point);
    }

    points
}

fn parse_input2(input: &str) -> Vec<(i64, i64)> {
    let mut points = Vec::new();

    let mut curr_point = (0, 0);
    for line in input.trim().lines() {
        let (_, _, hex_color) = line.split_whitespace().collect_tuple().unwrap();
        let hex_color = hex_color.trim_start_matches("(#").trim_end_matches(')');

        let (_, ins, val) = hex_color.rsplitn(3, "").collect_tuple().unwrap();

        let ins = ins.chars().next().unwrap();
        let val = i64::from_str_radix(val, 16).unwrap();

        curr_point = match ins {
            '0' => (curr_point.0 + val, curr_point.1),
            '2' => (curr_point.0 - val, curr_point.1),
            '3' => (curr_point.0, curr_point.1 + val),
            '1' => (curr_point.0, curr_point.1 - val),
            _ => panic!("Invalid instruction"),
        };

        points.push(curr_point);
    }

    points
}
