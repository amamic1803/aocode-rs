use crate::graph::{Graph, Vertex};
use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day09, 2015, 9, "All in a Single Night");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input).hamiltonian_path_min().0.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input).hamiltonian_path_max().0.to_string())
    }
}

fn parse_input(input: &str) -> Graph {
    let mut cities = HashMap::new();
    let mut city_index = 0;
    let mut edges = Vec::new();

    for line in input.trim().lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let city1 = line[0];
        let city2 = line[2];
        let distance = line[4].parse::<usize>().unwrap();
        edges.push((city1, city2, distance));
        if !cities.contains_key(city1) {
            cities.insert(city1, city_index);
            city_index += 1;
        }
        if !cities.contains_key(city2) {
            cities.insert(city2, city_index);
            city_index += 1;
        }
    }

    let mut graph = Graph::with_capacity(city_index);
    for i in 0..city_index {
        graph.add_vertex(Vertex::new(i));
    }

    for edge in edges {
        graph.set_edge_undirected(
            Vertex::new(cities[edge.0]),
            Vertex::new(cities[edge.1]),
            edge.2 as isize,
        );
    }

    graph
}
