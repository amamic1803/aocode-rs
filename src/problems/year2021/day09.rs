use crate::{Error, Solution};
use std::cmp::Reverse;

day!(Day09, 2021, 9, "Smoke Basin");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let cave = parse_input(input);
        let mut risks_sum = 0;

        for (i, row) in cave.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let up = if i > 0 { cave[i - 1][j] } else { 10 };
                let left = if j > 0 { cave[i][j - 1] } else { 10 };
                let right = if j < row.len() - 1 {
                    cave[i][j + 1]
                } else {
                    10
                };
                let down = if i < cave.len() - 1 {
                    cave[i + 1][j]
                } else {
                    10
                };

                if cell < up && cell < left && cell < right && cell < down {
                    risks_sum += cell as u32 + 1;
                }
            }
        }

        Ok(risks_sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let cave = parse_input(input);
        let mut visited_tiles = vec![vec![false; cave[0].len()]; cave.len()];
        let mut basin_sizes = Vec::new();

        let mut stack = Vec::new();
        for i in 0..cave.len() {
            for j in 0..cave[0].len() {
                if cave[i][j] != 9 && !visited_tiles[i][j] {
                    let mut basin_size = 0;
                    stack.clear();
                    stack.push((i, j));

                    while let Some((i, j)) = stack.pop() {
                        if !visited_tiles[i][j] {
                            if i > 0 && cave[i - 1][j] < 9 && !visited_tiles[i - 1][j] {
                                stack.push((i - 1, j));
                            };
                            if j > 0 && cave[i][j - 1] < 9 && !visited_tiles[i][j - 1] {
                                stack.push((i, j - 1));
                            };
                            if j < cave[0].len() - 1
                                && cave[i][j + 1] < 9
                                && !visited_tiles[i][j + 1]
                            {
                                stack.push((i, j + 1));
                            };
                            if i < cave.len() - 1 && cave[i + 1][j] < 9 && !visited_tiles[i + 1][j]
                            {
                                stack.push((i + 1, j));
                            };

                            visited_tiles[i][j] = true;
                            basin_size += 1;
                        }
                    }

                    basin_sizes.push(basin_size);
                }
            }
        }

        basin_sizes.sort_by_key(|&x| Reverse(x));
        Ok((basin_sizes[0] * basin_sizes[1] * basin_sizes[2]).to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut cave = Vec::new();
    input.lines().for_each(|line| {
        cave.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>(),
        )
    });
    cave
}
