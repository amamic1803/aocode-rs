use crate::{Error, Solution};
use ndlife::Life;

day!(Day17, 2020, 17, "Conway Cubes");

impl Solution for Day17 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let birth_rules = [3];
        let survival_rules = [2, 3];
        let mut life = Life::new(birth_rules, survival_rules).unwrap();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    life.set_cell(&[x as i64, y as i64, 0], true);
                }
            }
        }

        for _ in 0..6 {
            life.next_generation();
        }

        Ok(life.alive_cells().len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let birth_rules = [3];
        let survival_rules = [2, 3];
        let mut life = Life::new(birth_rules, survival_rules).unwrap();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    life.set_cell(&[x as i64, y as i64, 0, 0], true);
                }
            }
        }

        for _ in 0..6 {
            life.next_generation();
        }

        Ok(life.alive_cells().len().to_string())
    }
}
