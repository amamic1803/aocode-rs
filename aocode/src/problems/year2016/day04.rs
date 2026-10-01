use crate::{Error, Solution};
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashMap;

day!(Day04, 2016, 4, "Security Through Obscurity");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let rooms = filter_rooms(parse_input(input));
        Ok(rooms.iter().map(|(_, id, _)| id).sum::<usize>().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let rooms = filter_rooms(parse_input(input));
        Ok(rooms
            .iter()
            .find(|(name, id, _)| {
                let real_name = rotate_name(name, *id);
                real_name.contains("north")
                    && real_name.contains("pole")
                    && real_name.contains("object")
            })
            .unwrap()
            .1
            .to_string())
    }
}

fn filter_rooms(rooms: Vec<(&str, usize, [char; 5])>) -> Vec<(&str, usize, [char; 5])> {
    let mut filtered_rooms = Vec::new();

    for (name, id, checksum) in rooms {
        let mut char_counts = HashMap::new();
        for c in name.chars() {
            if c == '-' {
                continue;
            }
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        let mut char_counts: Vec<(char, usize)> = char_counts.into_iter().collect();
        char_counts.sort_by_key(|(c, count)| (Reverse(*count), *c));
        let new_checksum: [char; 5] = char_counts
            .iter()
            .map(|(c, _)| *c)
            .take(5)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        if new_checksum == checksum {
            filtered_rooms.push((name, id, checksum));
        }
    }

    filtered_rooms
}

fn rotate_name(name: &str, id: usize) -> String {
    let mut new_name = String::new();
    for c in name.chars() {
        if c == '-' {
            new_name.push(' ');
            continue;
        }
        let mut new_c = c;
        for _ in 0..id {
            new_c = match new_c {
                'z' => 'a',
                _ => char::from_u32(u32::from(new_c) + 1).unwrap(),
            }
        }
        new_name.push(new_c);
    }
    new_name.trim().to_string()
}

fn parse_input(input: &str) -> Vec<(&str, usize, [char; 5])> {
    let re_id = Regex::new(r"\d+").unwrap();
    let re_checksum = Regex::new(r"\[\w{5}]").unwrap();

    let mut rooms = Vec::new();

    for line in input.trim().lines() {
        let id_match = re_id.find(line).unwrap();

        let name = line.get(..id_match.start()).unwrap();
        let id = id_match.as_str().parse::<usize>().unwrap();
        let checksum: [char; 5] = re_checksum
            .find(line)
            .unwrap()
            .as_str()
            .chars()
            .enumerate()
            .filter(|(i, _)| *i > 0 && *i < 6)
            .map(|(_, c)| c)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        rooms.push((name, id, checksum));
    }

    rooms
}
