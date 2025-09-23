use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day24, 2017, 24, "Electromagnetic Moat");

impl Solution for Day24 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Components::new(input).strongest_bridge().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Components::new(input)
            .strongest_longest_bridge()
            .to_string())
    }
}

struct Components {
    components: Vec<(u16, u16)>,
    port_map: HashMap<u16, Vec<usize>>,
}
impl Components {
    fn new(input: &str) -> Self {
        let components = input
            .trim()
            .lines()
            .map(|line| {
                let mut line_parts = line.split('/');
                let a = line_parts.next().unwrap().parse::<u16>().unwrap();
                let b = line_parts.next().unwrap().parse::<u16>().unwrap();
                (a, b)
            })
            .collect::<Vec<_>>();

        let mut port_map = HashMap::new();
        for (i, component) in components.iter().enumerate() {
            port_map.entry(component.0).or_insert(Vec::new()).push(i);
            port_map.entry(component.1).or_insert(Vec::new()).push(i);
        }

        Self {
            components,
            port_map,
        }
    }

    fn strongest_bridge(&self) -> u16 {
        let mut components_used = vec![false; self.components.len()];
        let mut max_strength = 0;

        let mut apply = |current_strength, _| {
            if current_strength > max_strength {
                max_strength = current_strength;
            }
        };

        self.visit_bridges(0, 0, 0, &mut apply, &mut components_used);

        max_strength
    }

    fn strongest_longest_bridge(&self) -> u16 {
        let mut components_used = vec![false; self.components.len()];
        let mut max_strength = 0;
        let mut max_length = 0;

        let mut apply = |current_strength, current_length| {
            if current_length >= max_length && current_strength > max_strength {
                max_strength = current_strength;
                max_length = current_length;
            }
        };

        self.visit_bridges(0, 0, 0, &mut apply, &mut components_used);

        max_strength
    }

    fn visit_bridges<T>(
        &self,
        current_port: u16,
        current_strength: u16,
        current_length: u8,
        apply: &mut T,
        components_used: &mut [bool],
    ) where
        T: FnMut(u16, u8),
    {
        apply(current_strength, current_length);

        for next_component in self.port_map[&current_port].iter() {
            if !components_used[*next_component] {
                components_used[*next_component] = true;
                let (port1, port2) = self.components[*next_component];
                if port1 == current_port {
                    self.visit_bridges(
                        port2,
                        current_strength + port1 + port2,
                        current_length + 1,
                        apply,
                        components_used,
                    );
                } else {
                    self.visit_bridges(
                        port1,
                        current_strength + port1 + port2,
                        current_length + 1,
                        apply,
                        components_used,
                    );
                }
                components_used[*next_component] = false;
            }
        }
    }
}
