use crate::graph::{Graph, Vertex};
use crate::{Error, Solution};
use std::collections::VecDeque;

day!(Day24, 2016, 24, "Air Duct Spelunking");

impl Solution for Day24 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        // the problem is really
        // finding a minimal hamiltonian path with one edge fixed
        // or find the minimum among minimal hamiltonian paths between 2 fixed edges
        // while the second edge is fixed once as each vertex other than start

        let (numbers_locs, mut graph) = parse_input(input);

        Ok((0..numbers_locs.len())
            .skip(1)
            .map(|num| {
                graph
                    .hamiltonian_path_fixed_ends_min(Vertex::new(0), Vertex::new(num))
                    .0
            })
            .min()
            .unwrap()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        // this is a simpler problem than part1
        // just find a hamiltonian cycle

        let (_, graph) = parse_input(input);
        Ok(graph.hamiltonian_cycle_min().0.to_string())
    }
}

/// Parse input.
/// Returns the vector of locations of each number (index)
/// and the graph of distances between each number.
fn parse_input(input: &str) -> (Vec<(usize, usize)>, Graph) {
    let mut numbers_with_loc = Vec::new();
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => {
                    numbers_with_loc.push((c.to_digit(10).unwrap() as usize, (i, j)));
                    row.push(false);
                }
            }
        }
        grid.push(row);
    }
    numbers_with_loc.sort_by_key(|num_with_loc| num_with_loc.0);
    let numbers_locs = numbers_with_loc
        .into_iter()
        .map(|num_with_loc| num_with_loc.1)
        .collect::<Vec<_>>();

    let mut graph = Graph::with_capacity(numbers_locs.last().unwrap().0 + 1);
    for i in 0..numbers_locs.len() {
        graph.add_vertex(Vertex::new(i));
    }

    for i in 0..numbers_locs.len() {
        for j in (i + 1)..numbers_locs.len() {
            let distance = find_shortest_path(&grid, numbers_locs[i], numbers_locs[j]);
            graph.set_edge_undirected(Vertex::new(i), Vertex::new(j), distance as isize);
        }
    }

    (numbers_locs, graph)
}

fn find_shortest_path(grid: &[Vec<bool>], start: (usize, usize), end: (usize, usize)) -> usize {
    let mut visited = grid.to_vec();
    let mut distances = vec![vec![0; grid[0].len()]; grid.len()];

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(tile) = queue.pop_front() {
        if visited[tile.0][tile.1] {
            continue;
        }
        visited[tile.0][tile.1] = true;

        let next_distance = distances[tile.0][tile.1] + 1;

        if tile.0 > 0 && !visited[tile.0 - 1][tile.1] {
            distances[tile.0 - 1][tile.1] = next_distance;
            queue.push_back((tile.0 - 1, tile.1));
            if (tile.0 - 1, tile.1) == end {
                break;
            }
        }
        if tile.1 > 0 && !visited[tile.0][tile.1 - 1] {
            distances[tile.0][tile.1 - 1] = next_distance;
            queue.push_back((tile.0, tile.1 - 1));
            if (tile.0, tile.1 - 1) == end {
                break;
            }
        }
        if tile.0 < grid.len() - 1 && !visited[tile.0 + 1][tile.1] {
            distances[tile.0 + 1][tile.1] = next_distance;
            queue.push_back((tile.0 + 1, tile.1));
            if (tile.0 + 1, tile.1) == end {
                break;
            }
        }
        if tile.1 < grid[0].len() - 1 && !visited[tile.0][tile.1 + 1] {
            distances[tile.0][tile.1 + 1] = next_distance;
            queue.push_back((tile.0, tile.1 + 1));
            if (tile.0, tile.1 + 1) == end {
                break;
            }
        }
    }

    distances[end.0][end.1]
}
