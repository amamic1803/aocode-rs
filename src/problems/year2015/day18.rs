use crate::{Error, Solution};

day!(Day18, 2015, 18, "Like a GIF For Your Yard");

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut grid = parse_input(input);

        for _ in 0..100 {
            simulate_step(&mut grid);
        }

        let mut lights_on = 0;

        for row in &grid {
            for cell in row {
                if cell[0] {
                    lights_on += 1;
                }
            }
        }

        Ok(lights_on.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut grid = parse_input(input);
        grid[0][0][0] = true;
        grid[0][99][0] = true;
        grid[99][0][0] = true;
        grid[99][99][0] = true;

        for _ in 0..100 {
            simulate_step_2(&mut grid);
        }

        let mut lights_on = 0;

        for row in &grid {
            for cell in row {
                if cell[0] {
                    lights_on += 1;
                }
            }
        }

        Ok(lights_on.to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<[bool; 2]>> {
    let mut grid = Vec::with_capacity(100);

    for line in input.trim().lines() {
        let mut row = Vec::with_capacity(100);
        for c in line.chars() {
            match c {
                '#' => row.push([true, false]),
                '.' => row.push([false, false]),
                _ => panic!("Invalid character in input!"),
            }
        }
        grid.push(row);
    }

    assert_eq!(grid.len(), 100, "The grid should be 100x100");
    assert_eq!(grid[0].len(), 100, "The grid should be 100x100");

    grid
}

fn simulate_step(grid: &mut Vec<Vec<[bool; 2]>>) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let neighbors = get_neighbors(grid, (i, j));
            if grid[i][j][0] {
                grid[i][j][1] = neighbors == 2 || neighbors == 3;
            } else {
                grid[i][j][1] = neighbors == 3;
            }
        }
    }
    copy_new_to_old(grid);
}

fn simulate_step_2(grid: &mut Vec<Vec<[bool; 2]>>) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let neighbors = get_neighbors(grid, (i, j));
            if grid[i][j][0] {
                grid[i][j][1] = neighbors == 2 || neighbors == 3;
            } else {
                grid[i][j][1] = neighbors == 3;
            }
        }
    }

    grid[0][0][1] = true;
    grid[0][99][1] = true;
    grid[99][0][1] = true;
    grid[99][99][1] = true;

    copy_new_to_old(grid);
}

fn get_neighbors(grid: &[Vec<[bool; 2]>], loc: (usize, usize)) -> usize {
    let mut count = 0;

    let prev_row = loc.0.checked_sub(1);
    let next_row = if grid.get(loc.0 + 1).is_some() {
        Some(loc.0 + 1)
    } else {
        None
    };

    let prev_col = loc.1.checked_sub(1);
    let next_col = if grid[loc.0].get(loc.1 + 1).is_some() {
        Some(loc.1 + 1)
    } else {
        None
    };

    // right-side
    if let Some(right_col) = next_col {
        if grid[loc.0][right_col][0] {
            count += 1;
        };
        if let Some(up_row) = prev_row
            && grid[up_row][right_col][0]
        {
            count += 1
        }
        if let Some(down_row) = next_row
            && grid[down_row][right_col][0]
        {
            count += 1
        }
    }

    // left-side
    if let Some(left_col) = prev_col {
        if grid[loc.0][left_col][0] {
            count += 1;
        };
        if let Some(up_row) = prev_row
            && grid[up_row][left_col][0]
        {
            count += 1
        }
        if let Some(down_row) = next_row
            && grid[down_row][left_col][0]
        {
            count += 1
        }
    }

    // up
    if let Some(up_row) = prev_row
        && grid[up_row][loc.1][0]
    {
        count += 1
    }

    // down
    if let Some(down_row) = next_row
        && grid[down_row][loc.1][0]
    {
        count += 1
    }

    count
}

fn copy_new_to_old(grid: &mut Vec<Vec<[bool; 2]>>) {
    for row in grid {
        for cell in row {
            cell[0] = cell[1];
        }
    }
}
