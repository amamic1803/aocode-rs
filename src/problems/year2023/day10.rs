use crate::{Error, Solution};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

day!(Day10, 2023, 10, "Pipe Maze");

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let field = parse_input(input);

        // loop will have an even number of pipes, so we can divide by 2
        // (that is because for every move,
        // up/down/left/right we must do its opposite to get back to S)
        Ok((find_loop(&field).len() / 2).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut field = parse_input(input);

        let loop_tiles = find_loop(&field);

        let (s_coord, s_type) = determine_s(&field);
        field[s_coord.0][s_coord.1] = s_type;

        // expand loop tiles to 3x3 so that there are no "hidden" spaces between pipes,
        // then we will just flood fill the area outside the loop
        // and then finally from the full area of grid subtract area of loop pipes and flood-filled area
        // to get inside area

        let mut expanded_map = expand_loop(&loop_tiles, &field);

        flood_fill_outside(&mut expanded_map);

        // replace left dots with I (for inside)
        for row in expanded_map.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == '.' {
                    *cell = 'I';
                }
            }
        }

        let reduced_map = reduce_loop(&expanded_map);

        // show expanded and reduced maps
        // for row in expanded_map.iter() {
        //     for cell in row.iter() {
        //         print!("{}", cell);
        //     }
        //     println!();
        // }
        // for row in reduced_map.iter() {
        //     for cell in row.iter() {
        //         print!("{}", cell);
        //     }
        //     println!();
        // }

        let mut inside_count = 0;

        for row in reduced_map {
            for cell in row {
                if cell == 'I' {
                    inside_count += 1;
                }
            }
        }

        Ok(inside_count.to_string())
    }
}

static PIPE_EXPANSIONS: LazyLock<HashMap<char, [[char; 3]; 3]>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert('|', [['.', '#', '.'], ['.', '#', '.'], ['.', '#', '.']]);

    map.insert('-', [['.', '.', '.'], ['#', '#', '#'], ['.', '.', '.']]);

    map.insert('L', [['.', '#', '.'], ['.', '.', '#'], ['.', '.', '.']]);

    map.insert('J', [['.', '#', '.'], ['#', '.', '.'], ['.', '.', '.']]);

    map.insert('F', [['.', '.', '.'], ['.', '.', '#'], ['.', '#', '.']]);

    map.insert('7', [['.', '.', '.'], ['#', '.', '.'], ['.', '#', '.']]);

    map
});

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(field: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found")
}

fn find_adjacent_pipes(pipe: (usize, usize), field: &[Vec<char>]) -> [(usize, usize); 2] {
    match field[pipe.0][pipe.1] {
        'S' => {
            let mut found = 0;
            let mut result = [(0, 0); 2];

            if pipe.0 > 0 && ['|', 'F', '7'].contains(&field[pipe.0 - 1][pipe.1]) {
                result[found] = (pipe.0 - 1, pipe.1);
                found += 1;
            }

            if pipe.0 < field.len() - 1 && ['|', 'J', 'L'].contains(&field[pipe.0 + 1][pipe.1]) {
                result[found] = (pipe.0 + 1, pipe.1);
                found += 1;
            }

            if pipe.1 > 0 && ['-', 'L', 'F'].contains(&field[pipe.0][pipe.1 - 1]) {
                result[found] = (pipe.0, pipe.1 - 1);
                found += 1;
            }

            if pipe.1 < field[0].len() - 1 && ['-', '7', 'J'].contains(&field[pipe.0][pipe.1 + 1]) {
                result[found] = (pipe.0, pipe.1 + 1);
            }

            result
        }
        '|' => [(pipe.0 - 1, pipe.1), (pipe.0 + 1, pipe.1)],
        '-' => [(pipe.0, pipe.1 - 1), (pipe.0, pipe.1 + 1)],
        'L' => [(pipe.0, pipe.1 + 1), (pipe.0 - 1, pipe.1)],
        'J' => [(pipe.0, pipe.1 - 1), (pipe.0 - 1, pipe.1)],
        'F' => [(pipe.0, pipe.1 + 1), (pipe.0 + 1, pipe.1)],
        '7' => [(pipe.0, pipe.1 - 1), (pipe.0 + 1, pipe.1)],
        _ => panic!("Invalid pipe!"),
    }
}

fn find_loop(field: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();

    let mut current = find_start(field);
    let mut next = find_adjacent_pipes(current, field);
    visited.insert(current);

    while !(visited.contains(&next[0]) && visited.contains(&next[1])) {
        if visited.contains(&next[0]) {
            current = next[1];
            visited.insert(current);
            next = find_adjacent_pipes(current, field);
        } else {
            current = next[0];
            visited.insert(current);
            next = find_adjacent_pipes(current, field);
        }
    }

    visited
}

/// Finds the S and returns its coordinates and type
fn determine_s(field: &[Vec<char>]) -> ((usize, usize), char) {
    let s_coord = find_start(field);

    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    if s_coord.0 > 0 && ['|', 'F', '7'].contains(&field[s_coord.0 - 1][s_coord.1]) {
        up = true;
    }

    if s_coord.0 < field.len() - 1 && ['|', 'J', 'L'].contains(&field[s_coord.0 + 1][s_coord.1]) {
        down = true;
    }

    if s_coord.1 > 0 && ['-', 'L', 'F'].contains(&field[s_coord.0][s_coord.1 - 1]) {
        left = true;
    }

    if s_coord.1 < field[0].len() - 1 && ['-', '7', 'J'].contains(&field[s_coord.0][s_coord.1 + 1])
    {
        right = true;
    }

    let mut s_type = ' ';

    if up && down {
        s_type = '|';
    } else if left && right {
        s_type = '-';
    } else if up && right {
        s_type = 'L';
    } else if up && left {
        s_type = 'J';
    } else if down && left {
        s_type = '7';
    } else if down && right {
        s_type = 'F';
    }

    (s_coord, s_type)
}

/// Expands each tile to 3x3, only loop tiles are considered, others are considered empty
fn expand_loop(loop_tiles: &HashSet<(usize, usize)>, field: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; field[0].len() * 3]; field.len() * 3];

    for tile in loop_tiles {
        let expansion = PIPE_EXPANSIONS.get(&field[tile.0][tile.1]).unwrap();
        for i in (tile.0 * 3)..((tile.0 + 1) * 3) {
            for j in (tile.1 * 3)..((tile.1 + 1) * 3) {
                result[i][j] = expansion[i % 3][j % 3];
            }
        }
    }

    result
}

fn flood_fill_outside(expanded_field: &mut [Vec<char>]) {
    let mut current_tiles = BTreeSet::new();
    current_tiles.insert((0, 0));

    while let Some(current_tile) = current_tiles.pop_last() {
        expanded_field[current_tile.0][current_tile.1] = 'O';

        // up
        if current_tile.0 > 0 && expanded_field[current_tile.0 - 1][current_tile.1] == '.' {
            current_tiles.insert((current_tile.0 - 1, current_tile.1));
        }

        // down
        if current_tile.0 < expanded_field.len() - 1
            && expanded_field[current_tile.0 + 1][current_tile.1] == '.'
        {
            current_tiles.insert((current_tile.0 + 1, current_tile.1));
        }

        // left
        if current_tile.1 > 0 && expanded_field[current_tile.0][current_tile.1 - 1] == '.' {
            current_tiles.insert((current_tile.0, current_tile.1 - 1));
        }

        // right
        if current_tile.1 < expanded_field[0].len() - 1
            && expanded_field[current_tile.0][current_tile.1 + 1] == '.'
        {
            current_tiles.insert((current_tile.0, current_tile.1 + 1));
        }
    }
}

fn reduce_loop(expanded_field: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; expanded_field[0].len() / 3]; expanded_field.len() / 3];

    for i in 0..result.len() {
        for j in 0..result[0].len() {
            let mut all_i = true;
            let mut contains_pipe = false;

            for row in &expanded_field[(i * 3)..((i + 1) * 3)] {
                for element in &row[(j * 3)..((j + 1) * 3)] {
                    if *element != 'I' {
                        all_i = false;
                    }
                    if *element == '#' {
                        contains_pipe = true;
                    }
                }
            }

            if all_i {
                result[i][j] = 'I';
            } else if contains_pipe {
                result[i][j] = '#';
            }
        }
    }

    result
}
