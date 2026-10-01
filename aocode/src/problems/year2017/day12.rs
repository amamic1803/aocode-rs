use crate::{Error, Solution};

day!(Day12, 2017, 12, "Digital Plumber");

impl Solution for Day12 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut programs: Vec<Program> = input.lines().map(Program::new).collect();
        let mut to_visit = vec![0];

        while let Some(id) = to_visit.pop() {
            programs[id].visited = true;
            for &pipe in programs[id].pipes.iter() {
                if !programs[pipe].visited {
                    to_visit.push(pipe);
                }
            }
        }

        Ok(programs
            .into_iter()
            .filter(|p| p.visited)
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut programs: Vec<Program> = input.lines().map(Program::new).collect();
        let mut groups = 0;

        let mut to_visit = Vec::new();

        while let Some(start_id) = programs.iter().position(|p| !p.visited) {
            groups += 1;
            to_visit.push(start_id);

            while let Some(id) = to_visit.pop() {
                programs[id].visited = true;
                for &pipe in programs[id].pipes.iter() {
                    if !programs[pipe].visited {
                        to_visit.push(pipe);
                    }
                }
            }
        }

        Ok(groups.to_string())
    }
}

struct Program {
    visited: bool,
    pipes: Vec<usize>,
}
impl Program {
    fn new(input: &str) -> Self {
        let mut parts = input.split(" <-> ");
        parts.next().unwrap();
        let pipes = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            visited: false,
            pipes,
        }
    }
}
