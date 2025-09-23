use crate::graph::{Graph, Vertex};
use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day13, 2015, 13, "Knights of the Dinner Table");

impl Solution for Day13 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input).hamiltonian_cycle_max().0.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(parse_input(input).hamiltonian_path_max().0.to_string())
    }
}

fn parse_input(input: &str) -> Graph {
    let mut names = Vec::new();

    let mut weights_vec = Vec::new();
    for line in input.trim().lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let name1 = words[0];
        let name2 = words[10].trim_end_matches('.');
        let value = words[3].parse::<isize>().unwrap() * if words[2] == "gain" { 1 } else { -1 };

        if !names.contains(&name1) {
            names.push(name1);
        }
        if !names.contains(&name2) {
            names.push(name2);
        }

        weights_vec.push((name1, name2, value));
    }

    let mut weights = HashMap::new();
    for (name1, name2, value) in weights_vec {
        if weights.contains_key(&(name1, name2)) {
            *weights.get_mut(&(name1, name2)).unwrap() += value;
        } else if weights.contains_key(&(name2, name1)) {
            *weights.get_mut(&(name2, name1)).unwrap() += value;
        } else {
            weights.insert((name1, name2), value);
        }
    }

    let mut graph = Graph::with_capacity(names.len());
    for i in 0..names.len() {
        graph.add_vertex(Vertex::new(i));
    }
    for (key, val) in weights {
        let index1 = names.iter().position(|&x| x == key.0).unwrap();
        let index2 = names.iter().position(|&x| x == key.1).unwrap();
        graph.set_edge_undirected(Vertex::new(index1), Vertex::new(index2), val);
    }

    graph
}
