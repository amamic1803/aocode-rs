use crate::{Error, Solution};
use std::collections::HashSet;

day!(Day05, 2024, 5, "Print Queue");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (rules_set, updates) = parse_input(input);
        let mut middle_page_sum = 0;

        for update in updates.iter() {
            if update_correctly_ordered(&rules_set, update) {
                middle_page_sum += update[update.len() / 2] as u32;
            }
        }

        Ok(middle_page_sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (rules_set, mut updates) = parse_input(input);
        let mut middle_page_sum = 0;

        for update in updates.iter_mut() {
            let mut correctly_ordered = true;

            for i in 0..update.len() {
                for j in (i + 1)..update.len() {
                    if rules_set.contains(&(update[j], update[i])) {
                        correctly_ordered = false;
                        (update[i], update[j]) = (update[j], update[i]);
                    }
                }
            }

            if !correctly_ordered {
                middle_page_sum += update[update.len() / 2] as u32;
            }
        }

        Ok(middle_page_sum.to_string())
    }
}

fn parse_input(input: &str) -> (HashSet<(u16, u16)>, Vec<Vec<u16>>) {
    let mut rules_set = HashSet::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let mut parts = line.split('|').map(|n| n.parse::<u16>().unwrap());
            rules_set.insert((parts.next().unwrap(), parts.next().unwrap()));
        } else if line.contains(',') {
            updates.push(line.split(',').map(|n| n.parse::<u16>().unwrap()).collect());
        }
    }

    (rules_set, updates)
}

fn update_correctly_ordered(rules: &HashSet<(u16, u16)>, update: &[u16]) -> bool {
    for (i, n1) in update.iter().enumerate() {
        for n2 in update.iter().skip(i + 1) {
            if rules.contains(&(*n2, *n1)) {
                return false;
            }
        }
    }
    true
}
