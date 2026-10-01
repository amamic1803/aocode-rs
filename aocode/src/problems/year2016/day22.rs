use crate::{Error, Solution};
use std::collections::VecDeque;

day!(Day22, 2016, 22, "Grid Computing");

impl Solution for Day22 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Grid::new(input).viable_pairs().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Grid::new(input).move_data_fewest_steps().to_string())
    }
}

struct Grid {
    nodes: Vec<Vec<Node>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut nodes_list = Vec::new();
        for line in input.lines().skip(2) {
            nodes_list.push(Node::new(line));
        }

        let max_x = nodes_list.iter().map(|node| node.x).max().unwrap();
        let max_y = nodes_list.iter().map(|node| node.y).max().unwrap();

        let mut nodes =
            vec![vec![Node::new("/dev/grid/node-x0-y0 0T 0T 0T"); max_x + 1]; max_y + 1];

        for node in nodes_list.drain(..) {
            nodes[node.y][node.x] = node;
        }

        Self { nodes }
    }

    fn viable_pairs(&self) -> usize {
        let mut pairs = 0;
        for node in self.nodes.iter().flat_map(|row| row.iter()) {
            for node2 in self.nodes.iter().flat_map(|row| row.iter()) {
                if (node.x != node2.x || node.y != node2.y)
                    && node.used != 0
                    && node.used <= node2.avail
                {
                    pairs += 1;
                }
            }
        }
        pairs
    }

    fn move_data_fewest_steps(&self) -> i32 {
        let mut grid = vec![vec![false; self.nodes[0].len()]; self.nodes.len()];
        let mut empty_node = [0, 0];
        let data_node = [0, grid[0].len() - 1];
        grid[data_node[0]][data_node[1]] = true;

        let node_avg_size = self
            .nodes
            .iter()
            .flat_map(|row| row.iter())
            .map(|node| node.size)
            .sum::<u32>()
            / self.nodes.iter().flat_map(|row| row.iter()).count() as u32;
        for row in self.nodes.iter() {
            for node in row.iter() {
                if node.used == 0 {
                    empty_node = [node.y, node.x];
                } else if node.size > node_avg_size {
                    grid[node.y][node.x] = true;
                }
            }
        }

        fn shortest_path(
            start: [usize; 2],
            end: [usize; 2],
            grid: &[Vec<bool>],
        ) -> Vec<[usize; 2]> {
            if start == end {
                return Vec::new();
            }

            let mut visited = grid.to_vec();
            visited[start[0]][start[1]] = true;
            let mut distances = vec![vec![None; grid[0].len()]; grid.len()];
            distances[start[0]][start[1]] = Some(0);
            let mut queue = VecDeque::new();
            queue.push_back((start, 0));

            while let Some((tile, steps)) = queue.pop_front() {
                if tile[0] > 0 && !visited[tile[0] - 1][tile[1]] {
                    let next_tile = [tile[0] - 1, tile[1]];
                    visited[next_tile[0]][next_tile[1]] = true;
                    distances[next_tile[0]][next_tile[1]] = Some(steps + 1);
                    if next_tile == end {
                        break;
                    }
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[0] < grid.len() - 1 && !visited[tile[0] + 1][tile[1]] {
                    let next_tile = [tile[0] + 1, tile[1]];
                    visited[next_tile[0]][next_tile[1]] = true;
                    distances[next_tile[0]][next_tile[1]] = Some(steps + 1);
                    if next_tile == end {
                        break;
                    }
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[1] > 0 && !visited[tile[0]][tile[1] - 1] {
                    let next_tile = [tile[0], tile[1] - 1];
                    visited[next_tile[0]][next_tile[1]] = true;
                    distances[next_tile[0]][next_tile[1]] = Some(steps + 1);
                    if next_tile == end {
                        break;
                    }
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[1] < grid[0].len() - 1 && !visited[tile[0]][tile[1] + 1] {
                    let next_tile = [tile[0], tile[1] + 1];
                    visited[next_tile[0]][next_tile[1]] = true;
                    distances[next_tile[0]][next_tile[1]] = Some(steps + 1);
                    if next_tile == end {
                        break;
                    }
                    queue.push_back((next_tile, steps + 1));
                }
            }

            if distances[end[0]][end[1]].is_none() {
                panic!("No path found!");
            }

            let mut path = Vec::new();
            path.push(end);

            let mut current_tile = end;
            while current_tile != start {
                let current_distance = distances[current_tile[0]][current_tile[1]].unwrap();
                if current_tile[0] > 0
                    && distances[current_tile[0] - 1][current_tile[1]].unwrap_or(i32::MIN)
                        == current_distance - 1
                {
                    current_tile = [current_tile[0] - 1, current_tile[1]];
                } else if current_tile[0] < grid.len() - 1
                    && distances[current_tile[0] + 1][current_tile[1]].unwrap_or(i32::MIN)
                        == current_distance - 1
                {
                    current_tile = [current_tile[0] + 1, current_tile[1]];
                } else if current_tile[1] > 0
                    && distances[current_tile[0]][current_tile[1] - 1].unwrap_or(i32::MIN)
                        == current_distance - 1
                {
                    current_tile = [current_tile[0], current_tile[1] - 1];
                } else if current_tile[1] < grid[0].len() - 1
                    && distances[current_tile[0]][current_tile[1] + 1].unwrap_or(i32::MIN)
                        == current_distance - 1
                {
                    current_tile = [current_tile[0], current_tile[1] + 1];
                }
                path.push(current_tile);
            }

            path.reverse();

            path
        }

        fn shortest_path_length(start: [usize; 2], end: [usize; 2], grid: &[Vec<bool>]) -> i32 {
            if start == end {
                return 0;
            }

            let mut visited = grid.to_vec();
            visited[start[0]][start[1]] = true;
            let mut queue = VecDeque::new();
            queue.push_back((start, 0));

            while let Some((tile, steps)) = queue.pop_front() {
                if tile[0] > 0 && !visited[tile[0] - 1][tile[1]] {
                    let next_tile = [tile[0] - 1, tile[1]];
                    if next_tile == end {
                        return steps + 1;
                    }
                    visited[next_tile[0]][next_tile[1]] = true;
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[0] < grid.len() - 1 && !visited[tile[0] + 1][tile[1]] {
                    let next_tile = [tile[0] + 1, tile[1]];
                    if next_tile == end {
                        return steps + 1;
                    }
                    visited[next_tile[0]][next_tile[1]] = true;
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[1] > 0 && !visited[tile[0]][tile[1] - 1] {
                    let next_tile = [tile[0], tile[1] - 1];
                    if next_tile == end {
                        return steps + 1;
                    }
                    visited[next_tile[0]][next_tile[1]] = true;
                    queue.push_back((next_tile, steps + 1));
                }
                if tile[1] < grid[0].len() - 1 && !visited[tile[0]][tile[1] + 1] {
                    let next_tile = [tile[0], tile[1] + 1];
                    if next_tile == end {
                        return steps + 1;
                    }
                    visited[next_tile[0]][next_tile[1]] = true;
                    queue.push_back((next_tile, steps + 1));
                }
            }

            panic!("No path found!");
        }

        let mut steps = 0;
        let data_path = shortest_path(data_node, [0, 0], &grid);

        for i in 1..data_path.len() {
            steps += shortest_path_length(empty_node, data_path[i], &grid);
            grid[data_path[i][0]][data_path[i][1]] = true;
            grid[data_path[i - 1][0]][data_path[i - 1][1]] = false;
            empty_node = data_path[i - 1];
            steps += 1;
        }

        steps
    }
}

#[derive(Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
    avail: u32,
}
impl Node {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let path = parts.next().unwrap();
        let mut path_parts = path.split('-');
        let _ = path_parts.next().unwrap();
        let x = path_parts
            .next()
            .unwrap()
            .trim_start_matches('x')
            .parse()
            .unwrap();
        let y = path_parts
            .next()
            .unwrap()
            .trim_start_matches('y')
            .parse()
            .unwrap();

        let size = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
        let used = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
        let avail = parts.next().unwrap().trim_end_matches('T').parse().unwrap();

        Self {
            x,
            y,
            size,
            used,
            avail,
        }
    }
}
