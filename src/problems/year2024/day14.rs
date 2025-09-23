use crate::{Error, Solution};
use std::cmp::Ordering;
use std::sync::LazyLock;

day!(Day14, 2024, 14, "Restroom Redoubt");

impl Solution for Day14 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let robots = input.lines().map(Robot::from_string).collect::<Vec<_>>();
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for mut robot in robots {
            robot.simulate(100);

            match robot.x.cmp(&((GRID_WIDTH / 2) as i32)) {
                Ordering::Less => match robot.y.cmp(&((GRID_HEIGHT / 2) as i32)) {
                    Ordering::Less => {
                        q2 += 1;
                    }
                    Ordering::Equal => {} // ignore middle
                    Ordering::Greater => {
                        q3 += 1;
                    }
                },

                Ordering::Equal => {} // ignore middle

                Ordering::Greater => match robot.y.cmp(&((GRID_HEIGHT / 2) as i32)) {
                    Ordering::Less => {
                        q1 += 1;
                    }
                    Ordering::Equal => {} // ignore middle
                    Ordering::Greater => {
                        q4 += 1;
                    }
                },
            }
        }

        Ok((q1 * q2 * q3 * q4).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut robots = input.lines().map(Robot::from_string).collect::<Vec<_>>();

        let mut grid = vec![[false; GRID_WIDTH]; GRID_HEIGHT];
        for robot in &robots {
            grid[robot.y as usize][robot.x as usize] = true;
        }

        for i in 0.. {
            // To speed up finding a Christmas tree, we need to find a grid where more than 50%
            // of the cells have neighbors. When we find such a grid, only then we check if it contains
            // a Christmas tree
            let mut solo = 0;
            let mut neighbors = 0;
            for m in 0..grid.len() {
                for n in 0..grid[m].len() {
                    if grid[m][n] {
                        if (m > 0 && grid[m - 1][n])
                            || (m < grid.len() - 1 && grid[m + 1][n])
                            || (n > 0 && grid[m][n - 1])
                            || (n < grid[m].len() - 1 && grid[m][n + 1])
                        {
                            neighbors += 1;
                        } else {
                            solo += 1;
                        }
                    }
                }
            }
            let neighbor_percentage = neighbors as f32 / (neighbors + solo) as f32;
            if neighbor_percentage > 0.5 && contains_christmas_tree(&grid) {
                return Ok(i.to_string());
            }

            grid.fill([false; GRID_WIDTH]);
            for robot in robots.iter_mut() {
                robot.simulate(1);
                grid[robot.y as usize][robot.x as usize] = true;
            }
        }

        unreachable!()
    }
}

const GRID_WIDTH: usize = 101;
const GRID_HEIGHT: usize = 103;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
impl Robot {
    fn from_string(input: &str) -> Self {
        let (pos, vel) = input.trim().split_once(' ').unwrap();
        let pos = pos.trim_start_matches("p=");
        let vel = vel.trim_start_matches("v=");
        let (x, y) = pos.split_once(',').unwrap();
        let (vx, vy) = vel.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            vx: vx.parse().unwrap(),
            vy: vy.parse().unwrap(),
        }
    }
    fn simulate(&mut self, move_count: i32) {
        let dx = self.vx * move_count;
        let dy = self.vy * move_count;
        self.x = (self.x + dx).rem_euclid(GRID_WIDTH as i32);
        self.y = (self.y + dy).rem_euclid(GRID_HEIGHT as i32);
    }
}

fn contains_christmas_tree(grid: &[[bool; GRID_WIDTH]]) -> bool {
    static CHRISTMAS_TREE: LazyLock<Vec<Vec<bool>>> = LazyLock::new(|| {
        let tree_str = "
        ###############################
        #.............................#
        #.............................#
        #.............................#
        #.............................#
        #..............#..............#
        #.............###.............#
        #............#####............#
        #...........#######...........#
        #..........#########..........#
        #............#####............#
        #...........#######...........#
        #..........#########..........#
        #.........###########.........#
        #........#############........#
        #..........#########..........#
        #.........###########.........#
        #........#############........#
        #.......###############.......#
        #......#################......#
        #........#############........#
        #.......###############.......#
        #......#################......#
        #.....###################.....#
        #....#####################....#
        #.............###.............#
        #.............###.............#
        #.............###.............#
        #.............................#
        #.............................#
        #.............................#
        #.............................#
        ###############################
    ";
        let mut tree = Vec::new();
        for line in tree_str.trim().lines() {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                row.push(c == '#');
            }
            tree.push(row);
        }
        tree
    });

    for x in 0..GRID_WIDTH - CHRISTMAS_TREE[0].len() {
        for y in 0..GRID_HEIGHT - CHRISTMAS_TREE.len() {
            let mut found = true;
            'outer: for (i, row) in CHRISTMAS_TREE.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    if cell != grid[y + i][x + j] {
                        found = false;
                        break 'outer;
                    }
                }
            }
            if found {
                return true;
            }
        }
    }
    false
}
