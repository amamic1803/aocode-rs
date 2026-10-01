use crate::{Error, Solution};
use itertools::Itertools;
use std::collections::HashSet;

day!(Day25, 2017, 25, "The Halting Problem");

impl Solution for Day25 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().lines();
        let start_state = input
            .next()
            .unwrap()
            .trim_start_matches("Begin in state ")
            .trim_end_matches('.')
            .chars()
            .next()
            .unwrap();
        let steps = input
            .next()
            .unwrap()
            .trim_start_matches("Perform a diagnostic checksum after ")
            .trim_end_matches(" steps.")
            .parse()
            .unwrap();

        let input = input.join("\n");
        let states = State::parse_states(&input);

        let mut storage = HashSet::new();
        let mut cursor = 0;
        let mut state = states
            .iter()
            .position(|state| state.name == start_state)
            .unwrap();

        for _ in 0..steps {
            if storage.contains(&cursor) {
                // 1

                if !states[state].write[1] {
                    storage.remove(&cursor);
                }
                cursor += states[state].move_direction[1];
                state = states[state].next_state_index[1];
            } else {
                // 0

                if states[state].write[0] {
                    storage.insert(cursor);
                }
                cursor += states[state].move_direction[0];
                state = states[state].next_state_index[0];
            }
        }

        Ok(storage.len().to_string())
    }

    fn part2(&self, _input: &str) -> Result<String, Error> {
        Ok(String::from("Advent of Code 2017 solved!"))
    }
}

struct State {
    name: char,
    write: [bool; 2],
    move_direction: [i32; 2],
    next_state: [char; 2],
    next_state_index: [usize; 2],
}
impl State {
    fn parse_states(input: &str) -> Vec<Self> {
        let mut states = Vec::new();

        let mut name = ' ';
        let mut write = [false; 2];
        let mut move_direction = [0; 2];
        let mut next_state = [' '; 2];
        let next_state_index = [0; 2];

        let mut variant = 0;
        for line in input.trim().lines() {
            let line = line.trim();
            if line.starts_with("In state ") {
                name = line
                    .trim_start_matches("In state ")
                    .trim_end_matches(':')
                    .chars()
                    .next()
                    .unwrap();
            } else if line.starts_with("If the current value is ") {
                let value = line
                    .trim_start_matches("If the current value is ")
                    .trim_end_matches(':')
                    .parse::<u8>()
                    .unwrap();
                variant = value;
            } else if line.starts_with("- Write the value ") {
                let value = line
                    .trim_start_matches("- Write the value ")
                    .trim_end_matches('.')
                    .parse::<u8>()
                    .unwrap();
                write[variant as usize] = value == 1;
            } else if line.starts_with("- Move one slot to the ") {
                move_direction[variant as usize] = match line
                    .trim_start_matches("- Move one slot to the ")
                    .trim_end_matches('.')
                {
                    "right" => 1,
                    "left" => -1,
                    _ => {
                        panic!("Invalid move direction");
                    }
                };
            } else if line.starts_with("- Continue with state ") {
                next_state[variant as usize] = line
                    .trim_start_matches("- Continue with state ")
                    .trim_end_matches('.')
                    .chars()
                    .next()
                    .unwrap();
            } else if line.is_empty() {
                states.push(State {
                    name,
                    write,
                    move_direction,
                    next_state,
                    next_state_index,
                });
            }
        }

        states.push(State {
            name,
            write,
            move_direction,
            next_state,
            next_state_index,
        });

        for i in 0..states.len() {
            states[i].next_state_index[0] = states
                .iter()
                .position(|other| states[i].next_state[0] == other.name)
                .unwrap();
            states[i].next_state_index[1] = states
                .iter()
                .position(|other| states[i].next_state[1] == other.name)
                .unwrap();
        }

        states
    }
}
