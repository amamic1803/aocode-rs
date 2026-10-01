use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day21, 2017, 21, "Fractal Art");

impl Solution for Day21 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (rules_2x2, rules_3x3) = parse_rules(input);
        let mut grid = INITIAL_GRID
            .into_iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();

        for _ in 0..5 {
            enhance_image(&mut grid, &rules_2x2, &rules_3x3);
        }

        Ok(grid.iter().flatten().filter(|&&b| b).count().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (rules_2x2, rules_3x3) = parse_rules(input);
        let mut grid = INITIAL_GRID
            .into_iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();

        for _ in 0..18 {
            enhance_image(&mut grid, &rules_2x2, &rules_3x3);
        }

        Ok(grid.iter().flatten().filter(|&&b| b).count().to_string())
    }
}

type Rules2x2 = HashMap<[[bool; 2]; 2], [[bool; 3]; 3]>;
type Rules3x3 = HashMap<[[bool; 3]; 3], [[bool; 4]; 4]>;

const INITIAL_GRID: [[bool; 3]; 3] = [
    [false, true, false],
    [false, false, true],
    [true, true, true],
];

fn parse_rules(input: &str) -> (Rules2x2, Rules3x3) {
    let mut rules_2x2 = HashMap::with_capacity(2_usize.pow(4));
    let mut rules_2x2_candidates = Vec::new();
    let mut rules_3x3 = HashMap::with_capacity(2_usize.pow(9));
    let mut rules_3x3_candidates = Vec::new();

    for line in input.lines() {
        match line.chars().filter(|&c| c == '/').count() {
            3 => {
                let (key_str, value_str) = line.split_once(" => ").unwrap();
                let mut key = [[false; 2]; 2];
                let mut value = [[false; 3]; 3];
                for (i, row) in key_str.split('/').enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        key[i][j] = c == '#';
                    }
                }
                for (i, row) in value_str.split('/').enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        value[i][j] = c == '#';
                    }
                }

                let flip_horizontal = |mut field: [[bool; 2]; 2]| -> [[bool; 2]; 2] {
                    for row in field.iter_mut() {
                        row.reverse();
                    }
                    field
                };
                let flip_vertical = |mut field: [[bool; 2]; 2]| -> [[bool; 2]; 2] {
                    field.reverse();
                    field
                };
                let rotate = |field: [[bool; 2]; 2]| -> [[bool; 2]; 2] {
                    let mut new_field = [[false; 2]; 2];
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..field.len() {
                        for col in 0..field.len() {
                            new_field[col][field.len() - 1 - row] = field[row][col];
                        }
                    }
                    new_field
                };

                let rot_0 = key;
                let rot_1 = rotate(key);
                let rot_2 = rotate(rot_1);
                let rot_3 = rotate(rot_2);

                rules_2x2.insert(key, value);

                rules_2x2_candidates.push((rot_1, value));
                rules_2x2_candidates.push((rot_2, value));
                rules_2x2_candidates.push((rot_3, value));

                rules_2x2_candidates.push((flip_horizontal(rot_0), value));
                rules_2x2_candidates.push((flip_vertical(rot_0), value));
                rules_2x2_candidates.push((flip_horizontal(rot_1), value));
                rules_2x2_candidates.push((flip_vertical(rot_1), value));
                rules_2x2_candidates.push((flip_horizontal(rot_2), value));
                rules_2x2_candidates.push((flip_vertical(rot_2), value));
                rules_2x2_candidates.push((flip_horizontal(rot_3), value));
                rules_2x2_candidates.push((flip_vertical(rot_3), value));
            }
            5 => {
                let (key_str, value_str) = line.split_once(" => ").unwrap();
                let mut key = [[false; 3]; 3];
                let mut value = [[false; 4]; 4];
                for (i, row) in key_str.split('/').enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        key[i][j] = c == '#';
                    }
                }
                for (i, row) in value_str.split('/').enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        value[i][j] = c == '#';
                    }
                }

                let flip_hor = |mut field: [[bool; 3]; 3]| -> [[bool; 3]; 3] {
                    for row in field.iter_mut() {
                        row.reverse();
                    }
                    field
                };

                let flip_ver = |mut field: [[bool; 3]; 3]| -> [[bool; 3]; 3] {
                    field.reverse();
                    field
                };

                let rotate = |field: [[bool; 3]; 3]| -> [[bool; 3]; 3] {
                    let mut new_field = [[false; 3]; 3];
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..field.len() {
                        for col in 0..field.len() {
                            new_field[col][field.len() - 1 - row] = field[row][col];
                        }
                    }
                    new_field
                };

                let rot_0 = key;
                let rot_1 = rotate(key);
                let rot_2 = rotate(rot_1);
                let rot_3 = rotate(rot_2);

                rules_3x3.insert(key, value);

                rules_3x3_candidates.push((rot_1, value));
                rules_3x3_candidates.push((rot_2, value));
                rules_3x3_candidates.push((rot_3, value));

                rules_3x3_candidates.push((flip_hor(rot_0), value));
                rules_3x3_candidates.push((flip_ver(rot_0), value));
                rules_3x3_candidates.push((flip_hor(rot_1), value));
                rules_3x3_candidates.push((flip_ver(rot_1), value));
                rules_3x3_candidates.push((flip_hor(rot_2), value));
                rules_3x3_candidates.push((flip_ver(rot_2), value));
                rules_3x3_candidates.push((flip_hor(rot_3), value));
                rules_3x3_candidates.push((flip_ver(rot_3), value));
            }
            _ => panic!("Invalid input"),
        }
    }

    for rule in rules_2x2_candidates {
        rules_2x2.entry(rule.0).or_insert(rule.1);
    }
    for rule in rules_3x3_candidates {
        rules_3x3.entry(rule.0).or_insert(rule.1);
    }

    (rules_2x2, rules_3x3)
}

fn enhance_image(grid: &mut Vec<Vec<bool>>, rules_2x2: &Rules2x2, rules_3x3: &Rules3x3) {
    if grid.len().is_multiple_of(2) {
        let new_size = grid.len() / 2 * 3;
        let mut new_grid = vec![vec![false; new_size]; new_size];

        for (i, x) in (0..grid.len()).step_by(2).enumerate() {
            for (j, y) in (0..grid.len()).step_by(2).enumerate() {
                let key = [
                    [grid[x][y], grid[x][y + 1]],
                    [grid[x + 1][y], grid[x + 1][y + 1]],
                ];
                let value = *rules_2x2.get(&key).unwrap();

                for k in 0..3 {
                    for l in 0..3 {
                        new_grid[i * 3 + k][j * 3 + l] = value[k][l];
                    }
                }
            }
        }

        *grid = new_grid;
    } else if grid.len().is_multiple_of(3) {
        let new_size = grid.len() / 3 * 4;
        let mut new_grid = vec![vec![false; new_size]; new_size];

        for (i, x) in (0..grid.len()).step_by(3).enumerate() {
            for (j, y) in (0..grid.len()).step_by(3).enumerate() {
                let key = [
                    [grid[x][y], grid[x][y + 1], grid[x][y + 2]],
                    [grid[x + 1][y], grid[x + 1][y + 1], grid[x + 1][y + 2]],
                    [grid[x + 2][y], grid[x + 2][y + 1], grid[x + 2][y + 2]],
                ];
                let value = *rules_3x3.get(&key).unwrap();

                for k in 0..4 {
                    for l in 0..4 {
                        new_grid[i * 4 + k][j * 4 + l] = value[k][l];
                    }
                }
            }
        }

        *grid = new_grid;
    } else {
        panic!("Invalid grid size");
    }
}
