use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day12, 2018, 12, "Subterranean Sustainability");

impl Solution for Day12 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut pots = Pots::new(input);
        pots.simulate_generations(GENERATIONS1);
        Ok(pots.sum().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut pots = Pots::new(input);
        pots.simulate_generations(GENERATIONS2);
        Ok(pots.sum().to_string())
    }
}

const GENERATIONS1: u64 = 20;
const GENERATIONS2: u64 = 50_000_000_000;

struct Pots {
    relevant: Vec<i64>,
    first_pot: i64,
    old_relevant: Vec<i64>,
    old_first_pot: i64,
    rules: HashMap<[bool; 5], bool>,
}
impl Pots {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let mut initial_state: Vec<bool> = lines
            .next()
            .unwrap()
            .trim_start_matches("initial state: ")
            .chars()
            .map(|c| c == '#')
            .collect();
        let mut first_pot = 0;
        while !initial_state[0] {
            initial_state.remove(0);
            first_pot += 1;
        }
        while !initial_state[initial_state.len() - 1] {
            initial_state.pop();
        }
        let mut relevant = Vec::new();
        for (i, b) in initial_state.iter().enumerate() {
            if *b {
                relevant.push(i as i64);
            }
        }

        lines.next();

        let mut rules = HashMap::new();
        for line in lines {
            let (key_str, val_str) = line.split_once(" => ").unwrap();
            let mut key = [false; 5];
            for (i, c) in key_str.chars().enumerate() {
                key[i] = c == '#';
            }
            let val = val_str == "#";
            rules.insert(key, val);
        }

        Self {
            relevant,
            first_pot,
            old_relevant: Vec::new(),
            old_first_pot: 0,
            rules,
        }
    }

    fn sum(&self) -> i64 {
        let mut sum = 0;
        for i in self.relevant.iter() {
            sum += i + self.first_pot;
        }
        sum
    }

    fn next_generation(&mut self) {
        self.old_relevant.clear();
        self.old_relevant.extend(self.relevant.iter());
        self.old_first_pot = self.first_pot;

        self.relevant.clear();
        for i in (self.old_relevant.first().unwrap() - 2)..(self.old_relevant.last().unwrap() + 2) {
            let mut key = [false; 5];
            for (ind, j) in (-2..=2).enumerate() {
                key[ind] = self.old_relevant.contains(&(i + j));
            }
            if *self.rules.get(&key).unwrap_or(&false) {
                self.relevant.push(i);
            }
        }

        let norm = 0 - self.relevant[0];
        self.relevant.iter_mut().for_each(|i| *i += norm);
        self.first_pot -= norm;
    }

    fn simulate_generations(&mut self, mut generations: u64) {
        while generations > 0 {
            self.next_generation();
            generations -= 1;
            if self.old_relevant == self.relevant {
                let diff = self.first_pot - self.old_first_pot;
                let total_diff = diff * generations as i64;
                self.first_pot += total_diff;
                break;
            }
        }
    }
}
