use crate::{Error, Solution};
use std::collections::BTreeSet;

day!(Day18, 2022, 18, "Boiling Boulders");

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let cubes = parse_input(input);

        let mut cubes_count = 0;
        for cube_i in &cubes {
            for cube_j in cube_i {
                for cube_k in cube_j {
                    if *cube_k == Position::Cube {
                        cubes_count += 1;
                    }
                }
            }
        }

        let mut result = 6 * cubes_count; // 6 faces of the cube

        // check for cubes that are adjacent to each other and their neighbouring faces from the result

        for x in 0..cubes.len() {
            for y in 0..cubes[x].len() {
                for z in 0..cubes[x][y].len() {
                    if cubes[x][y][z] == Position::Cube {
                        // check each of the 6 faces of the cube, we don't need bound checks because the outer layer is empty so the cube is always surrounded by empty space
                        if cubes[x - 1][y][z] == Position::Cube {
                            result -= 1;
                        }
                        if cubes[x + 1][y][z] == Position::Cube {
                            result -= 1;
                        }
                        if cubes[x][y - 1][z] == Position::Cube {
                            result -= 1;
                        }
                        if cubes[x][y + 1][z] == Position::Cube {
                            result -= 1;
                        }
                        if cubes[x][y][z - 1] == Position::Cube {
                            result -= 1;
                        }
                        if cubes[x][y][z + 1] == Position::Cube {
                            result -= 1;
                        }
                    }
                }
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut cubes = parse_input(input);
        let mut result = 0;

        // we will do flood fill from outside, and each time adjacent cubes are found, we will add their face to the result
        let mut encountered_pos = BTreeSet::new();
        encountered_pos.insert((0, 0, 0));

        while let Some(current_pos) = encountered_pos.pop_last() {
            cubes[current_pos.0][current_pos.1][current_pos.2] = Position::Filled;

            if current_pos.0 > 0 {
                match cubes[current_pos.0 - 1][current_pos.1][current_pos.2] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0 - 1, current_pos.1, current_pos.2));
                    }
                    Position::Filled => {}
                }
            }
            if current_pos.0 < cubes.len() - 1 {
                match cubes[current_pos.0 + 1][current_pos.1][current_pos.2] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0 + 1, current_pos.1, current_pos.2));
                    }
                    Position::Filled => {}
                }
            }

            if current_pos.1 > 0 {
                match cubes[current_pos.0][current_pos.1 - 1][current_pos.2] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0, current_pos.1 - 1, current_pos.2));
                    }
                    Position::Filled => {}
                }
            }
            if current_pos.1 < cubes[current_pos.0].len() - 1 {
                match cubes[current_pos.0][current_pos.1 + 1][current_pos.2] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0, current_pos.1 + 1, current_pos.2));
                    }
                    Position::Filled => {}
                }
            }

            if current_pos.2 > 0 {
                match cubes[current_pos.0][current_pos.1][current_pos.2 - 1] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0, current_pos.1, current_pos.2 - 1));
                    }
                    Position::Filled => {}
                }
            }
            if current_pos.2 < cubes[current_pos.0][current_pos.1].len() - 1 {
                match cubes[current_pos.0][current_pos.1][current_pos.2 + 1] {
                    Position::Cube => {
                        result += 1;
                    }
                    Position::Empty => {
                        encountered_pos.insert((current_pos.0, current_pos.1, current_pos.2 + 1));
                    }
                    Position::Filled => {}
                }
            }
        }

        Ok(result.to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Position>>> {
    let mut cubes_coords = input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|coord| coord.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // offset all cubes by 1 in each direction
    // because we want the first row, column and slice to be empty for part 2
    for coord in cubes_coords.iter_mut() {
        for c in coord.iter_mut() {
            *c += 1;
        }
    }

    // find max coord to determine size of grid
    let mut max = cubes_coords[0][0];
    for coords in &cubes_coords {
        for coord in coords {
            if *coord > max {
                max = *coord;
            }
        }
    }

    // if max coordinate is n, the grid size is n + 1,
    // but we want the grid size to be n + 2 so that the last row, column and slice are also empty
    let grid_size = max + 2;

    let mut cubes = vec![vec![vec![Position::Empty; grid_size]; grid_size]; grid_size];

    for coords in cubes_coords {
        cubes[coords[0]][coords[1]][coords[2]] = Position::Cube;
    }

    cubes
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Position {
    Empty,
    Cube,
    Filled,
}
