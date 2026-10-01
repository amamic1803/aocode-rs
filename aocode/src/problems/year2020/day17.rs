use crate::{Error, Solution};
use ndlife::Life;
use std::collections::HashSet;

day!(Day17, 2020, 17, "Conway Cubes");

impl Solution for Day17 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let birth_rules = [3].into_iter().collect();
        let survival_rules = [2, 3].into_iter().collect();
        let mut alive_cells = HashSet::new();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    alive_cells.insert([x as i64, y as i64, 0]);
                }
            }
        }

        let mut life =
            Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
        for _ in 0..6 {
            life.next_generation();
        }

        Ok(life.alive_cells().len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let birth_rules = [3].into_iter().collect();
        let survival_rules = [2, 3].into_iter().collect();
        let mut alive_cells = HashSet::new();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    alive_cells.insert([x as i64, y as i64, 0, 0]);
                }
            }
        }

        let mut life =
            Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
        for _ in 0..6 {
            life.next_generation();
        }

        Ok(life.alive_cells().len().to_string())
    }
}
