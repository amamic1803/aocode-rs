use crate::{Error, Solution};
use regex::Regex;
use std::cmp::{max, min};
use std::fmt::Write;

day!(Day14, 2022, 14, "Regolith Reservoir");

impl Solution for Day14 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Cave::new(input).simulate_sand().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().to_owned();
        let re = Regex::new(r",(\d+)").unwrap();
        let max_y = re
            .captures_iter(&input)
            .map(|cap| cap.get(1).unwrap().as_str().parse::<i32>().unwrap())
            .max()
            .unwrap();
        let platform_y = max_y + 2;
        writeln!(
            &mut input,
            "\n{},{} -> {},{}",
            SAND_DROP_X - platform_y,
            platform_y,
            SAND_DROP_X + platform_y,
            platform_y
        )
        .unwrap();
        Ok(Cave::new(&input).simulate_sand().to_string())
    }
}

/// X coordinate where the grains of sand are dropped
const SAND_DROP_X: i32 = 500;

struct Cave {
    grid: Vec<Vec<char>>,
    x_offset: i32,
}
impl Cave {
    /// Create a new cave from the input
    fn new(input: &str) -> Self {
        let mut structures = Vec::new();
        let mut max_y = 0;
        let mut min_x = SAND_DROP_X;
        let mut max_x = SAND_DROP_X;
        for line in input.lines() {
            let mut structure = Vec::new();
            for point in line.split(" -> ") {
                let (x, y) = point.trim().split_once(',').unwrap();
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                structure.push((x, y));
            }
            structures.push(structure);
        }
        min_x -= 1;
        max_x += 1;
        max_y += 1; // Add one to max_y so the last row is empty for the detection of the abyss
        let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];
        for structure in structures {
            for i in 0..(structure.len() - 1) {
                if structure[i].0 == structure[i + 1].0 {
                    // vertical line
                    for y in min(structure[i].1, structure[i + 1].1)
                        ..=max(structure[i].1, structure[i + 1].1)
                    {
                        grid[y as usize][(structure[i].0 - min_x) as usize] = '#';
                    }
                } else {
                    // horizontal line
                    for x in min(structure[i].0, structure[i + 1].0)
                        ..=max(structure[i].0, structure[i + 1].0)
                    {
                        grid[structure[i].1 as usize][(x - min_x) as usize] = '#';
                    }
                }
            }
        }

        Self {
            grid,
            x_offset: min_x,
        }
    }

    /// Drop a new grain of sand
    /// # Returns
    /// True if a grain of sand came to rest, false if it fell into the abyss or couldn't even be created
    fn drop_sand(&mut self) -> bool {
        let mut new_sand = (SAND_DROP_X - self.x_offset, 0);
        if self.grid[new_sand.1][new_sand.0 as usize] != '.' {
            return false;
        }
        while new_sand.1 < self.grid.len() - 1 {
            if self.grid[new_sand.1 + 1][new_sand.0 as usize] == '.' {
                new_sand.1 += 1;
            } else if self.grid[new_sand.1 + 1][new_sand.0 as usize - 1] == '.' {
                new_sand = (new_sand.0 - 1, new_sand.1 + 1);
            } else if self.grid[new_sand.1 + 1][new_sand.0 as usize + 1] == '.' {
                new_sand = (new_sand.0 + 1, new_sand.1 + 1);
            } else {
                self.grid[new_sand.1][new_sand.0 as usize] = 'o';
                break;
            }
        }
        new_sand.1 != self.grid.len() - 1
    }

    /// Simulate the flow of sand.
    /// Simulation ends when the grain of sand falls into the abyss or can't be created.
    /// # Returns
    /// The number of grains of sand that came to rest before the simulation ended
    fn simulate_sand(&mut self) -> i32 {
        let mut i = 0;
        while self.drop_sand() {
            i += 1;
        }
        i
    }
}
