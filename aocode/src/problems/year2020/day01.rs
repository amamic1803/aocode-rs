use crate::{Error, Solution};

day!(Day01, 2020, 1, "Report Repair");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let set = store_input_in_set(input);
        for num in &set {
            let diff = 2020 - num;
            if set.contains(&diff) {
                return Ok((num * diff).to_string());
            }
        }
        Err(Error::NoSolution)
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let set = store_input_in_set(input);
        for (i, num1) in set.iter().enumerate() {
            let diff1 = 2020 - num1;
            for (j, num2) in set.iter().enumerate() {
                if i == j || num2 > &diff1 {
                    continue;
                }
                let diff2 = diff1 - num2;
                if set.contains(&diff2) {
                    return Ok((num1 * num2 * diff2).to_string());
                }
            }
        }
        Err(Error::NoSolution)
    }
}

fn store_input_in_set(input: &str) -> Vec<usize> {
    let mut set = Vec::new();
    for line in input.trim().lines() {
        set.push(line.parse::<usize>().unwrap());
    }
    set
}
