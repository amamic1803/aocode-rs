use crate::{Error, Solution};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

day!(Day18, 2024, 18, "RAM Run");

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut grid = vec![[true; 71]; 71];
        for line in input.lines().take(1024) {
            let (n1, n2) = line.split_once(',').unwrap();
            let n1 = n1.parse::<usize>().unwrap(); // distance from left
            let n2 = n2.parse::<usize>().unwrap(); // distance from top
            grid[n2][n1] = false;
        }
        let start = (0, 0);
        let target = (70, 70);

        Ok(dijkstra(&grid, start, target).unwrap().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut grid = vec![[true; 71]; 71];
        let mut lines = input.lines();
        for _ in 0..1024 {
            if let Some(line) = lines.next() {
                let (n1, n2) = line.split_once(',').unwrap();
                let n1 = n1.parse::<usize>().unwrap(); // distance from left
                let n2 = n2.parse::<usize>().unwrap(); // distance from top
                grid[n2][n1] = false;
            }
        }
        for line in lines {
            let (n1, n2) = line.split_once(',').unwrap();
            let n1 = n1.parse::<usize>().unwrap(); // distance from left
            let n2 = n2.parse::<usize>().unwrap(); // distance from top
            grid[n2][n1] = false;
            if dijkstra(&grid, (0, 0), (70, 70)).is_none() {
                return Ok(format!("{},{}", n1, n2));
            }
        }

        Err(Error::NoSolution)
    }
}

fn dijkstra<const N: usize>(
    grid: &[[bool; N]],
    start: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    #[derive(Eq, PartialEq)]
    struct Node {
        coords: (usize, usize),
        weight: usize,
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight).reverse()
        }
    }

    let mut weights = vec![[usize::MAX; N]; grid.len()];
    weights[start.0][start.1] = 0;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Node {
        coords: start,
        weight: 0,
    });

    while let Some(node) = min_heap.pop() {
        if node.coords == target {
            return Some(node.weight);
        }
        if node.weight > weights[node.coords.0][node.coords.1] {
            continue;
        }
        let (i, j) = node.coords;
        if i > 0 && grid[i - 1][j] && node.weight + 1 < weights[i - 1][j] {
            weights[i - 1][j] = node.weight + 1;
            min_heap.push(Node {
                coords: (i - 1, j),
                weight: node.weight + 1,
            });
        }
        if i < grid.len() - 1 && grid[i + 1][j] && node.weight + 1 < weights[i + 1][j] {
            weights[i + 1][j] = node.weight + 1;
            min_heap.push(Node {
                coords: (i + 1, j),
                weight: node.weight + 1,
            });
        }
        if j > 0 && grid[i][j - 1] && node.weight + 1 < weights[i][j - 1] {
            weights[i][j - 1] = node.weight + 1;
            min_heap.push(Node {
                coords: (i, j - 1),
                weight: node.weight + 1,
            });
        }
        if j < N - 1 && grid[i][j + 1] && node.weight + 1 < weights[i][j + 1] {
            weights[i][j + 1] = node.weight + 1;
            min_heap.push(Node {
                coords: (i, j + 1),
                weight: node.weight + 1,
            });
        }
    }

    None
}
