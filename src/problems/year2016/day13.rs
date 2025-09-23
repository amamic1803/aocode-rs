use crate::{Error, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

day!(Day13, 2016, 13, "A Maze of Twisty Little Cubicles");

impl Solution for Day13 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let favorite_number = input.trim().parse::<usize>().unwrap();
        let mut maze = Maze::new(favorite_number);
        Ok(maze.find_path(START, END).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let favorite_number = input.trim().parse::<usize>().unwrap();
        let mut maze = Maze::new(favorite_number);
        Ok(maze.reachable_locations(START, STEP_LIMIT).to_string())
    }
}

const START: (usize, usize) = (1, 1);
const END: (usize, usize) = (31, 39);
const STEP_LIMIT: usize = 50;

// true = open, false = wall
struct Maze {
    favorite_number: usize,
    maze: HashMap<(usize, usize), bool>,
}
impl Maze {
    fn new(favorite_number: usize) -> Self {
        Self {
            favorite_number,
            maze: HashMap::new(),
        }
    }

    fn get(&mut self, position: (isize, isize)) -> bool {
        if position.0 < 0 || position.1 < 0 {
            false
        } else {
            let position = (position.0 as usize, position.1 as usize);
            if let Some(value) = self.maze.get(&position) {
                *value
            } else {
                let value = self.check_position(position);
                self.maze.insert(position, value);
                value
            }
        }
    }

    fn check_position(&self, position: (usize, usize)) -> bool {
        let mut value = position.0.pow(2)
            + 3 * position.0
            + 2 * position.0 * position.1
            + position.1
            + position.1.pow(2)
            + self.favorite_number;
        let mut number_of_ones = 0;

        while value != 0 {
            number_of_ones += value & 1;
            value >>= 1;
        }

        number_of_ones % 2 == 0
    }

    /// Returns the number of steps to get from start to end
    fn find_path(&mut self, start: (usize, usize), end: (usize, usize)) -> usize {
        let start = (start.0 as isize, start.1 as isize);
        let end = (end.0 as isize, end.1 as isize);
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        let mut steps = 0;

        to_visit.push_back(start);

        'outer_loop: while !to_visit.is_empty() {
            let step_len = to_visit.len();

            for _ in 0..step_len {
                let working_pos = to_visit.pop_front().unwrap();

                if visited.contains(&working_pos) {
                    continue;
                }

                if working_pos == end {
                    break 'outer_loop;
                }

                visited.insert(working_pos);

                let up = (working_pos.0, working_pos.1 - 1);
                let down = (working_pos.0, working_pos.1 + 1);
                let left = (working_pos.0 - 1, working_pos.1);
                let right = (working_pos.0 + 1, working_pos.1);

                if self.get(up) && !visited.contains(&up) {
                    to_visit.push_back(up);
                }
                if self.get(down) && !visited.contains(&down) {
                    to_visit.push_back(down);
                }
                if self.get(left) && !visited.contains(&left) {
                    to_visit.push_back(left);
                }
                if self.get(right) && !visited.contains(&right) {
                    to_visit.push_back(right);
                }
            }

            steps += 1;
        }

        steps
    }

    /// Returns the number of locations that can be reached in the given number of steps
    fn reachable_locations(&mut self, start: (usize, usize), step_limit: usize) -> usize {
        let start = (start.0 as isize, start.1 as isize);
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        let mut steps = 0;

        to_visit.push_back(start);

        // smaller than or equal to because the starting position doesn't count as a step
        while !to_visit.is_empty() && steps <= step_limit {
            let step_len = to_visit.len();

            for _ in 0..step_len {
                let working_pos = to_visit.pop_front().unwrap();

                if visited.contains(&working_pos) {
                    continue;
                }

                visited.insert(working_pos);

                let up = (working_pos.0, working_pos.1 - 1);
                let down = (working_pos.0, working_pos.1 + 1);
                let left = (working_pos.0 - 1, working_pos.1);
                let right = (working_pos.0 + 1, working_pos.1);

                if self.get(up) && !visited.contains(&up) {
                    to_visit.push_back(up);
                }
                if self.get(down) && !visited.contains(&down) {
                    to_visit.push_back(down);
                }
                if self.get(left) && !visited.contains(&left) {
                    to_visit.push_back(left);
                }
                if self.get(right) && !visited.contains(&right) {
                    to_visit.push_back(right);
                }
            }

            steps += 1;
        }

        visited.len()
    }
}
