use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day01, 2024, 1, "Historian Hysteria");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        input.lines().for_each(|line| {
            let mut line_split = line.split_whitespace();
            list1.push(line_split.next().unwrap().parse::<u32>().unwrap());
            list2.push(line_split.next().unwrap().parse::<u32>().unwrap());
        });
        list1.sort();
        list2.sort();
        Ok(list1
            .into_iter()
            .zip(list2)
            .map(|(a, b)| a.abs_diff(b))
            .sum::<u32>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut list1 = Vec::new();
        let mut map2 = HashMap::new();
        input.lines().for_each(|line| {
            let mut line_split = line.split_whitespace();
            list1.push(line_split.next().unwrap().parse::<u32>().unwrap());
            let num2 = line_split.next().unwrap().parse::<u32>().unwrap();
            *map2.entry(num2).or_insert(0) += 1;
        });
        Ok(list1
            .into_iter()
            .map(|n| n * map2.get(&n).unwrap_or(&0))
            .sum::<u32>()
            .to_string())
    }
}
