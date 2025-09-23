use crate::{Error, Solution};
use itertools::Itertools;

day!(Day24, 2015, 24, "It Hangs in the Balance");

impl Solution for Day24 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let weights = input
            .trim()
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let total_weight: usize = weights.iter().sum();
        let group_weight = total_weight / 3;
        let mut min_qe = usize::MAX;

        for i in 0..(weights.len() / 3) {
            let mut found = false;
            for combination in weights.iter().combinations(i) {
                if combination.iter().copied().sum::<usize>() == group_weight {
                    found = true;
                    let qe = combination.into_iter().product::<usize>();
                    if qe < min_qe {
                        min_qe = qe;
                    }
                }
            }
            if found {
                break;
            }
        }

        Ok(min_qe.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let weights = input
            .trim()
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let total_weight: usize = weights.iter().sum();
        let group_weight = total_weight / 4;
        let mut min_qe = usize::MAX;

        for i in 0..(weights.len() / 4) {
            let mut found = false;
            for combination in weights.iter().combinations(i) {
                if combination.iter().copied().sum::<usize>() == group_weight {
                    found = true;
                    let qe = combination.into_iter().product::<usize>();
                    if qe < min_qe {
                        min_qe = qe;
                    }
                }
            }
            if found {
                break;
            }
        }

        Ok(min_qe.to_string())
    }
}
