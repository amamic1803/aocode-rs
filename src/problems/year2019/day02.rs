use crate::{Error, Solution};
use itertools::Itertools;

day!(Day02, 2019, 2, "1202 Program Alarm");

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut array = parse_input(input);
        array[1] = 12;
        array[2] = 2;

        simulate(&mut array);

        Ok(array[0].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let wanted_output = 19690720;

        let input_array = parse_input(input);

        for (m, n) in (0..100).cartesian_product(0..100) {
            let mut array = input_array.clone();
            array[1] = m;
            array[2] = n;

            simulate(&mut array);

            if array[0] == wanted_output {
                return Ok((100 * m + n).to_string());
            }
        }

        Err(Error::NoSolution)
    }
}

fn simulate(array: &mut [usize]) {
    for i in (0..(array.len() - 3)).step_by(4) {
        match array[i] {
            1 => {
                let pos1 = array[i + 1];
                let pos2 = array[i + 2];
                let pos3 = array[i + 3];
                array[pos3] = array[pos1] + array[pos2];
            }
            2 => {
                let pos1 = array[i + 1];
                let pos2 = array[i + 2];
                let pos3 = array[i + 3];
                array[pos3] = array[pos1] * array[pos2];
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
