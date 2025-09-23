use crate::{Error, Solution};
use std::collections::{HashMap, HashSet};

day!(Day22, 2017, 22, "Sporifica Virus");

impl Solution for Day22 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut virus = Virus::new(input);
        for _ in 0..10_000 {
            virus.burst();
        }
        Ok(virus.infected_count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut virus_evolved = VirusEvolved::new(input);
        for _ in 0..10_000_000 {
            virus_evolved.burst();
        }
        Ok(virus_evolved.infected_count.to_string())
    }
}

struct Virus {
    infected_nodes: HashSet<(i32, i32)>,
    current_node: (i32, i32),
    direction: u8, // 0 = up, 1 = right, 2 = down, 3 = left
    infected_count: u32,
}
impl Virus {
    fn new(input: &str) -> Self {
        let mut infected = HashSet::new();

        let center = (
            input.lines().next().unwrap().chars().count() as i32 / 2,
            input.lines().count() as i32 / 2,
        );

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    infected.insert((x as i32 - center.0, y as i32 - center.1));
                }
            }
        }

        Self {
            infected_nodes: infected,
            current_node: (0, 0),
            direction: 0,
            infected_count: 0,
        }
    }

    fn burst(&mut self) {
        if self.infected_nodes.contains(&self.current_node) {
            self.infected_nodes.remove(&self.current_node);
            self.direction = (self.direction + 1) % 4;
        } else {
            self.infected_nodes.insert(self.current_node);
            self.direction = (self.direction + 3) % 4;
            self.infected_count += 1;
        }

        match self.direction {
            0 => self.current_node.1 -= 1,
            1 => self.current_node.0 += 1,
            2 => self.current_node.1 += 1,
            3 => self.current_node.0 -= 1,
            _ => unreachable!(),
        }
    }
}

struct VirusEvolved {
    affected_nodes: HashMap<(i32, i32), NodeState>,
    current_node: (i32, i32),
    direction: u8, // 0 = up, 1 = right, 2 = down, 3 = left
    infected_count: u32,
}
impl VirusEvolved {
    fn new(input: &str) -> Self {
        let mut affected_nodes = HashMap::new();

        let center = (
            input.lines().next().unwrap().chars().count() as i32 / 2,
            input.lines().count() as i32 / 2,
        );

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    affected_nodes.insert(
                        (x as i32 - center.0, y as i32 - center.1),
                        NodeState::Infected,
                    );
                }
            }
        }

        Self {
            affected_nodes,
            current_node: (0, 0),
            direction: 0,
            infected_count: 0,
        }
    }

    fn burst(&mut self) {
        match self.affected_nodes.get(&self.current_node) {
            Some(NodeState::Infected) => {
                self.affected_nodes
                    .insert(self.current_node, NodeState::Flagged);
                self.direction = (self.direction + 1) % 4;
            }
            Some(NodeState::Flagged) => {
                self.affected_nodes.remove(&self.current_node);
                self.direction = (self.direction + 2) % 4;
            }
            Some(NodeState::Weakened) => {
                self.affected_nodes
                    .insert(self.current_node, NodeState::Infected);
                self.infected_count += 1;
            }
            None => {
                self.affected_nodes
                    .insert(self.current_node, NodeState::Weakened);
                self.direction = (self.direction + 3) % 4;
            }
        }

        match self.direction {
            0 => self.current_node.1 -= 1,
            1 => self.current_node.0 += 1,
            2 => self.current_node.1 += 1,
            3 => self.current_node.0 -= 1,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum NodeState {
    Weakened,
    Infected,
    Flagged,
}
