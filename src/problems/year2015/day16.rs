use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day16, 2015, 16, "Aunt Sue");

impl Solution for Day16 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let sues = parse_input(input);
        let wanted_sue = wanted_sue();

        for (i, sue) in sues.iter().enumerate() {
            let mut good_match = true;
            for (key, val) in wanted_sue.iter() {
                if sue.contains_key(key) && sue[key] != *val {
                    good_match = false;
                }
            }

            if good_match {
                return Ok((i + 1).to_string());
            }
        }

        Err(Error::NoSolution)
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let sues = parse_input(input);
        let wanted_sue = wanted_sue();

        for (i, sue) in sues.iter().enumerate() {
            let mut good_match = true;
            for (key, val) in wanted_sue.iter() {
                if sue.contains_key(key) {
                    match *key {
                        "cats" | "trees" => {
                            if sue[key] <= *val {
                                good_match = false;
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if sue[key] >= *val {
                                good_match = false;
                            }
                        }
                        _ => {
                            if sue[key] != *val {
                                good_match = false;
                            }
                        }
                    }
                }
            }

            if good_match {
                return Ok((i + 1).to_string());
            }
        }

        Err(Error::NoSolution)
    }
}

fn wanted_sue() -> HashMap<&'static str, usize> {
    let mut wanted_sue = HashMap::new();

    wanted_sue.insert("children", 3);
    wanted_sue.insert("cats", 7);
    wanted_sue.insert("samoyeds", 2);
    wanted_sue.insert("pomeranians", 3);
    wanted_sue.insert("akitas", 0);
    wanted_sue.insert("vizslas", 0);
    wanted_sue.insert("goldfish", 5);
    wanted_sue.insert("trees", 3);
    wanted_sue.insert("cars", 2);
    wanted_sue.insert("perfumes", 1);

    wanted_sue
}

fn parse_input(input: &str) -> Vec<HashMap<&str, usize>> {
    let mut sues = Vec::with_capacity(500);

    for line in input.trim().lines() {
        let mut new_sue = HashMap::new();
        let parsed_line: Vec<&str> = line.split_whitespace().collect();

        new_sue.insert(
            parsed_line[2].trim_end_matches(':'),
            parsed_line[3]
                .trim_end_matches(',')
                .parse::<usize>()
                .unwrap(),
        );
        new_sue.insert(
            parsed_line[4].trim_end_matches(':'),
            parsed_line[5]
                .trim_end_matches(',')
                .parse::<usize>()
                .unwrap(),
        );
        new_sue.insert(
            parsed_line[6].trim_end_matches(':'),
            parsed_line[7]
                .trim_end_matches(',')
                .parse::<usize>()
                .unwrap(),
        );

        sues.push(new_sue);
    }

    assert_eq!(sues.len(), 500, "There should be 500 sues");

    sues
}
