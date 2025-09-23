use crate::{Error, Solution};

day!(Day08, 2015, 8, "Matchsticks");

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut result = 0;
        for line in input.trim().lines() {
            let (literal_len, memory_len) = string_len(line.trim());
            result += literal_len - memory_len;
        }
        Ok(result.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut result: usize = 0;
        for line in input.trim().lines() {
            let (literal_len, memory_len) = encode_len(line.trim());
            result += literal_len - memory_len;
        }
        Ok(result.to_string())
    }
}

fn encode_len(string: &str) -> (usize, usize) {
    let memory_len = string.chars().count();
    let mut literal_len = memory_len + 2;

    literal_len += string.matches('\\').count();
    literal_len += string.matches('\"').count();

    (literal_len, memory_len)
}

fn string_len(string: &str) -> (usize, usize) {
    let literal_len = string.chars().count();
    let mut memory_len = literal_len - 2;

    let mut skip: usize = 0;
    for (i, c) in string.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if c == '\\' {
            if string.chars().nth(i + 1).unwrap() == 'x' {
                skip = 3;
                memory_len -= 3;
            } else {
                skip = 1;
                memory_len -= 1;
            }
        }
    }

    (literal_len, memory_len)
}
