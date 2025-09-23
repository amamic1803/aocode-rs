use crate::{Error, Solution};
use regex::Regex;
use std::sync::LazyLock;

day!(Day09, 2016, 9, "Explosives in Cyberspace");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut file = String::from(input.trim());
        let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
        let mut position = 0;

        while !file.is_empty() {
            let re_match = match re.find_at(&file, position) {
                Some(caps) => caps,
                None => break,
            };
            let start = re_match.start();
            let range = re_match.range();

            let len = re_match
                .as_str()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split('x')
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let times = re_match
                .as_str()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split('x')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            file.replace_range(range, "");
            let compressed_str = file.get(start..(start + len)).unwrap();
            file.insert_str(start, &compressed_str.repeat(times - 1));

            position = start + len * times;
        }

        Ok(file.chars().count().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(decompress(input.trim()).to_string())
    }
}

fn decompress(mut current_string: &str) -> u64 {
    // assuming that no marker constructs a new marker while decompressing
    // seems to work for the input, I guess the input is intentionally constructed like this

    let mut file_len: u64 = 0;
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((\d+)x(\d+)\)").unwrap());

    while !current_string.is_empty() {
        let re_match = match RE.find(current_string) {
            Some(caps) => caps,
            None => {
                file_len += current_string.chars().count() as u64;
                break;
            }
        };
        let start = re_match.start();
        let end = re_match.end();

        let len = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let times = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .next_back()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        file_len += start as u64;
        file_len += decompress(&current_string[end..(end + len)]) * times as u64;

        current_string = &current_string[(end + len)..];
    }

    file_len
}
