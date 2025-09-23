use crate::{Error, Solution};
use std::cmp::{max, min};
use std::iter::zip;

day!(Day05, 2021, 5, "Hydrothermal Venture");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut lines = parse_input(input);

        // eliminate diagonal lines
        lines.retain(|line| line[0][0] == line[1][0] || line[0][1] == line[1][1]);

        // find the size of the grid
        let grid_size = grid_size(&lines);

        // create the grid
        let mut grid = vec![vec![0; grid_size[0]]; grid_size[1]];

        // draw lines
        for line in &lines {
            draw_line(&mut grid, line);
        }

        // count how many points are covered by 2 or more lines
        Ok(covered_by_2(&grid).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let lines = parse_input(input);

        // find the size of the grid
        let grid_size = grid_size(&lines);

        // create the grid
        let mut grid = vec![vec![0; grid_size[0]]; grid_size[1]];

        // draw lines
        for line in &lines {
            draw_line(&mut grid, line);
        }

        // count how many points are covered by 2 or more lines
        Ok(covered_by_2(&grid).to_string())
    }
}

fn parse_input(input: &str) -> Vec<[[u32; 2]; 2]> {
    let mut lines = Vec::new();

    for line in input.trim().lines() {
        let mut coords = [[0; 2]; 2];

        for (i, coord_pair) in line.split("->").enumerate() {
            let (coord_x, coord_y) = coord_pair.split_once(',').unwrap();
            coords[i][0] = coord_x.trim().parse().unwrap();
            coords[i][1] = coord_y.trim().parse().unwrap();
        }

        lines.push(coords);
    }

    lines
}

fn covered_by_2(grid: &Vec<Vec<u32>>) -> u32 {
    // count how many points are covered by 2 or more lines
    let mut points_count = 0;
    for row in grid {
        for point in row {
            if *point >= 2 {
                points_count += 1;
            }
        }
    }

    // return the number of points
    points_count
}

fn grid_size(lines: &Vec<[[u32; 2]; 2]>) -> [usize; 2] {
    let mut max_coord_val: [usize; 2] = [0, 0];
    for line in lines {
        for coord in line {
            for i in 0..=1 {
                if coord[i] as usize > max_coord_val[i] {
                    max_coord_val[i] = coord[i] as usize;
                }
            }
        }
    }
    max_coord_val[0] += 1;
    max_coord_val[1] += 1;

    max_coord_val
}

fn draw_line(grid: &mut [Vec<u32>], line: &[[u32; 2]; 2]) {
    // horizontal line (y1 == y2)
    if line[0][1] == line[1][1] {
        grid[line[0][1] as usize]
            [min(line[0][0], line[1][0]) as usize..=max(line[0][0], line[1][0]) as usize]
            .iter_mut()
            .for_each(|point| *point += 1);
    } else if line[0][0] == line[1][0] {
        // vertical line (x1 == x2)
        grid[min(line[0][1], line[1][1]) as usize..=max(line[0][1], line[1][1]) as usize]
            .iter_mut()
            .for_each(|row| row[line[0][0] as usize] += 1);
    } else {
        // diagonal line
        let x_iter = if line[0][0] < line[1][0] {
            (line[0][0]..=line[1][0]).collect::<Vec<_>>()
        } else {
            (line[1][0]..=line[0][0]).rev().collect()
        };
        let y_iter = if line[0][1] < line[1][1] {
            (line[0][1]..=line[1][1]).collect::<Vec<_>>()
        } else {
            (line[1][1]..=line[0][1]).rev().collect()
        };

        for (x, y) in zip(x_iter, y_iter) {
            grid[y as usize][x as usize] += 1;
        }
    }
}
