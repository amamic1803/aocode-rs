use crate::{Error, Solution};
use std::collections::HashSet;

day!(Day14, 2023, 14, "Parabolic Reflector Dish");

impl Solution for Day14 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut platform = Platform::new(input);
        platform.tilt_up();
        Ok(platform.load().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut platform = Platform::new(input);
        platform.spin_cycle();
        Ok(platform.load().to_string())
    }
}

const CYCLES: u32 = 1_000_000_000;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum RockType {
    Round, // movable rock
    Cube,  // immovable rock
    Empty, // no rock
}

struct Platform {
    rocks: Vec<Vec<RockType>>,
}
impl Platform {
    fn new(input: &str) -> Self {
        let rocks = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => RockType::Round,
                        '#' => RockType::Cube,
                        '.' => RockType::Empty,
                        _ => panic!("Invalid rock type"),
                    })
                    .collect()
            })
            .collect();

        Self { rocks }
    }

    fn tilt_up(&mut self) {
        self.transpose();
        self.tilt_left();
        self.transpose();
    }

    fn tilt_down(&mut self) {
        self.transpose();
        self.tilt_right();
        self.transpose();
    }

    fn tilt_left(&mut self) {
        for row in self.rocks.iter_mut() {
            let mut empty_pos = 0;
            for i in 0..row.len() {
                match row[i] {
                    RockType::Empty => {}
                    RockType::Round => {
                        row[i] = RockType::Empty;
                        row[empty_pos] = RockType::Round;
                        empty_pos += 1;
                    }
                    RockType::Cube => empty_pos = i + 1,
                }
            }
        }
    }

    fn tilt_right(&mut self) {
        for row in self.rocks.iter_mut() {
            let mut empty_pos = row.len() - 1;
            for i in (0..row.len()).rev() {
                match row[i] {
                    RockType::Empty => {}
                    RockType::Round => {
                        row[i] = RockType::Empty;
                        row[empty_pos] = RockType::Round;
                        empty_pos = empty_pos.saturating_sub(1);
                    }
                    RockType::Cube => empty_pos = i.saturating_sub(1),
                }
            }
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }

    fn spin_cycle(&mut self) {
        let mut seen_states_vector = Vec::new();
        let mut seen_states_set = HashSet::new();
        seen_states_vector.push(self.rocks.clone());
        seen_states_set.insert(self.rocks.clone());

        for _ in 0..CYCLES {
            self.tilt_cycle();

            if !seen_states_set.insert(self.rocks.clone()) {
                // this means that set already contained this state and we have found a cycle
                break;
            } else {
                // if we haven't seen this state before, add it to the vector
                seen_states_vector.push(self.rocks.clone());
            }
        }

        let cycle_start = seen_states_vector
            .iter()
            .position(|state| state == &self.rocks)
            .unwrap();
        let cycle_length = seen_states_vector.len() - cycle_start;

        let cycle_index = ((CYCLES - cycle_start as u32) % cycle_length as u32) as usize;

        self.rocks
            .clone_from(&seen_states_vector[cycle_start + cycle_index]);
    }

    fn load(&self) -> u64 {
        let mut load = 0;

        for (i, row) in self.rocks.iter().enumerate() {
            load += (row.iter().filter(|&&rock| rock == RockType::Round).count() * (row.len() - i))
                as u64;
        }

        load
    }

    fn transpose(&mut self) {
        let mut new_rocks = vec![vec![RockType::Empty; self.rocks.len()]; self.rocks[0].len()];
        for (i, row) in self.rocks.iter().enumerate() {
            for (j, rock) in row.iter().enumerate() {
                new_rocks[j][i] = *rock;
            }
        }
        self.rocks = new_rocks;
    }
}
