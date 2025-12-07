use crate::{Error, Solution};

day!(Day04, 2025, 4, "Printing Department");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let grid = input.lines().map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] {
                    let mut count2 = 0;

                    if r > 0 {
                        if c > 0 && grid[r - 1][c - 1] { count2 += 1; }
                        if grid[r - 1][c] { count2 += 1; }
                        if c + 1 < grid[r].len() && grid[r - 1][c + 1] { count2 += 1; }
                    }
                    if c > 0 && grid[r][c - 1] { count2 += 1; }
                    if c + 1 < grid[r].len() && grid[r][c + 1] { count2 += 1; }
                    if r + 1 < grid.len() {
                        if c > 0 && grid[r + 1][c - 1] { count2 += 1; }
                        if grid[r + 1][c] { count2 += 1; }
                        if c + 1 < grid[r].len() && grid[r + 1][c + 1] { count2 += 1; }
                    }

                    if count2 < 4 {
                        count += 1;
                    }
                }
            }
        }
        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut grid = input.lines().map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut removed = 0;
        let mut changed = true;
        while changed {
            changed = false;
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] {
                        let mut count2 = 0;

                        if r > 0 {
                            if c > 0 && grid[r - 1][c - 1] { count2 += 1; }
                            if grid[r - 1][c] { count2 += 1; }
                            if c + 1 < grid[r].len() && grid[r - 1][c + 1] { count2 += 1; }
                        }
                        if c > 0 && grid[r][c - 1] { count2 += 1; }
                        if c + 1 < grid[r].len() && grid[r][c + 1] { count2 += 1; }
                        if r + 1 < grid.len() {
                            if c > 0 && grid[r + 1][c - 1] { count2 += 1; }
                            if grid[r + 1][c] { count2 += 1; }
                            if c + 1 < grid[r].len() && grid[r + 1][c + 1] { count2 += 1; }
                        }

                        if count2 < 4 {
                            changed = true;
                            grid[r][c] = false;
                            removed += 1;
                        }
                    }
                }
            }
        }
        Ok(removed.to_string())
    }
}
