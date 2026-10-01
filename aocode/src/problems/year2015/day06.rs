use crate::{Error, Solution};

day!(Day06, 2015, 6, "Probably a Fire Hazard");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let instructions = parse_input(input);
        let mut grid = vec![vec![false; 1000]; 1000];

        for ins in instructions {
            execute_instruction(&mut grid, ins);
        }

        Ok(lights_on(&grid).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let instructions = parse_input(input);
        let mut grid = vec![vec![0; 1000]; 1000];

        for ins in instructions {
            execute_instruction2(&mut grid, ins);
        }

        Ok(brightness(&grid).to_string())
    }
}

type Instruction = (u8, (usize, usize), (usize, usize));

fn parse_input(input: &str) -> Vec<Instruction> {
    // (instruction, lower, upper)
    // on = 1, off = 0, toggle = 2

    let mut result = Vec::new();
    let mut line_vec: Vec<&str>;

    for line in input.trim().lines() {
        line_vec = line.split(' ').collect();

        match line_vec[1] {
            "on" => {
                let mut split = line_vec[2].split(',');
                let mut split2 = line_vec[4].split(',');
                result.push((
                    1,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
            "off" => {
                let mut split = line_vec[2].split(',');
                let mut split2 = line_vec[4].split(',');
                result.push((
                    0,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
            _ => {
                let mut split = line_vec[1].split(',');
                let mut split2 = line_vec[3].split(',');
                result.push((
                    2,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
        }
    }

    result
}

fn execute_instruction(grid: &mut [Vec<bool>], ins: Instruction) {
    for row in grid.iter_mut().take(ins.2.0 + 1).skip(ins.1.0) {
        for cell in row.iter_mut().take(ins.2.1 + 1).skip(ins.1.1) {
            match ins.0 {
                0 => *cell = false,
                1 => *cell = true,
                2 => *cell = !*cell,
                _ => unreachable!("Invalid instruction"),
            }
        }
    }
}

fn execute_instruction2(grid: &mut [Vec<isize>], ins: Instruction) {
    for row in grid.iter_mut().take(ins.2.0 + 1).skip(ins.1.0) {
        for cell in row.iter_mut().take(ins.2.1 + 1).skip(ins.1.1) {
            match ins.0 {
                0 => {
                    if *cell > 0 {
                        *cell -= 1;
                    }
                }
                1 => *cell += 1,
                2 => *cell += 2,
                _ => unreachable!("Invalid instruction"),
            }
        }
    }
}

fn lights_on(grid: &[Vec<bool>]) -> usize {
    let mut result = 0;

    for row in grid {
        for cell in row {
            if *cell {
                result += 1;
            }
        }
    }

    result
}

fn brightness(grid: &[Vec<isize>]) -> usize {
    let mut result: usize = 0;

    for row in grid {
        for cell in row {
            result += usize::try_from(*cell).unwrap();
        }
    }

    result
}
