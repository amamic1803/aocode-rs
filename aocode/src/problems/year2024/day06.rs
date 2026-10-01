use crate::{Error, Solution};
use std::collections::HashSet;

day!(Day06, 2024, 6, "Guard Gallivant");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (mut location, mut grid) = parse_input(input);

        while let Some(new_location) = simulate_move(&mut grid, location) {
            location = new_location;
        }

        Ok(grid
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|&c| c == 'X')
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (start_location, mut grid) = parse_input(input);
        let obstacle_grid = {
            let mut grid = grid.clone();
            let mut location = start_location;
            while let Some(new_location) = simulate_move(&mut grid, location) {
                location = new_location;
            }
            grid[start_location.i][start_location.j] = '.'; // can't put an obstacle on the starting location
            grid
        };
        let mut visited_locations = HashSet::new();
        let mut distinct_obstacles_locations = 0;

        for (i, row) in obstacle_grid.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if cell == 'X' {
                    visited_locations.clear(); // clear visited locations
                    grid[i][j] = '#'; // put an obstacle

                    let mut cycle = false;
                    let mut location = start_location;
                    while let Some(new_location) = simulate_move(&mut grid, location) {
                        visited_locations.insert(location);
                        if visited_locations.contains(&new_location) {
                            cycle = true;
                            break;
                        } else {
                            location = new_location;
                        }
                    }

                    if cycle {
                        distinct_obstacles_locations += 1;
                    }

                    grid[i][j] = '.'; // remove the obstacle
                }
            }
        }

        Ok(distinct_obstacles_locations.to_string())
    }
}

fn parse_input(input: &str) -> (Location, Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut location = Location {
        i: 0,
        j: 0,
        direction: Direction::Up,
    };

    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' | '.' => row.push(c),
                '^' => {
                    location.i = i;
                    location.j = j;
                    row.push('X');
                }
                'v' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Down;
                    row.push('X');
                }
                '<' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Left;
                    row.push('X');
                }
                '>' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Right;
                    row.push('X');
                }
                _ => panic!("Invalid character in input"),
            }
        }
        grid.push(row);
    }

    (location, grid)
}

fn simulate_move(grid: &mut [Vec<char>], mut location: Location) -> Option<Location> {
    match location.direction {
        Direction::Up => {
            if location.i == 0 {
                return None;
            }
            if grid[location.i - 1][location.j] == '#' {
                location.direction = Direction::Right;
            } else {
                location.i -= 1;
                grid[location.i][location.j] = 'X';
            }
        }
        Direction::Down => {
            if location.i == grid.len() - 1 {
                return None;
            }
            if grid[location.i + 1][location.j] == '#' {
                location.direction = Direction::Left;
            } else {
                location.i += 1;
                grid[location.i][location.j] = 'X';
            }
        }
        Direction::Left => {
            if location.j == 0 {
                return None;
            }
            if grid[location.i][location.j - 1] == '#' {
                location.direction = Direction::Up;
            } else {
                location.j -= 1;
                grid[location.i][location.j] = 'X';
            }
        }
        Direction::Right => {
            if location.j == grid[0].len() - 1 {
                return None;
            }
            if grid[location.i][location.j + 1] == '#' {
                location.direction = Direction::Down;
            } else {
                location.j += 1;
                grid[location.i][location.j] = 'X';
            }
        }
    }

    Some(location)
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Location {
    i: usize,
    j: usize,
    direction: Direction,
}
