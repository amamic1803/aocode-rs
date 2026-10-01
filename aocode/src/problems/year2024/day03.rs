use crate::{Error, Solution};
use regex::Regex;
use std::sync::LazyLock;

day!(Day03, 2024, 3, "Mull It Over");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(execute_muls(input).to_string())
    }

    fn part2(&self, mut input: &str) -> Result<String, Error> {
        let mut total_result = 0;

        let do_re = Regex::new(r"do\(\)").unwrap(); // do()
        let dont_re = Regex::new(r"don't\(\)").unwrap(); // don't()

        while !input.is_empty() {
            let next_dont = if let Some(next_dont) = dont_re.find(input) {
                next_dont
            } else {
                total_result += execute_muls(input);
                break;
            };
            let (do_part, rest) = input.split_at(next_dont.end());
            let (do_part, _) = do_part.split_at(next_dont.start());
            total_result += execute_muls(do_part);
            if let Some(next_do) = do_re.find(rest) {
                let (_, rest) = rest.split_at(next_do.end());
                input = rest;
            } else {
                break;
            };
        }

        Ok(total_result.to_string())
    }
}

fn execute_muls(hay: &str) -> u32 {
    static MUL_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((?<n1>[0-9]{1,3}),(?<n2>[0-9]{1,3})\)").unwrap()); // mul(1,2)

    let results = MUL_RE.captures_iter(hay).map(|caps| {
        let n1 = caps.name("n1").unwrap().as_str().parse::<u32>().unwrap();
        let n2 = caps.name("n2").unwrap().as_str().parse::<u32>().unwrap();
        n1 * n2
    });

    results.sum()
}
