use crate::{Error, Solution};
use regex::Regex;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet, VecDeque};

day!(Day11, 2016, 11, "Radioisotope Thermoelectric Generators");

impl Solution for Day11 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        // get initial building state
        let state = State::from_input(input);

        // setup wanted building state
        let mut wanted_state = State::new();
        wanted_state.elevator = 3;
        for _ in 0..state.elements.len() {
            wanted_state.elements.push([3, 3]);
        }

        // search for an optimal solution
        match bfs_search(state, wanted_state) {
            Some(steps) => Ok(steps.to_string()),
            None => Err(Error::NoSolution),
        }
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        // get initial building state
        let mut state = State::from_input(input);
        state.elements.push([0, 0]);
        state.elements.push([0, 0]);
        state.elements.sort();

        // setup wanted building state
        let mut wanted_state = State::new();
        wanted_state.elevator = 3;
        for _ in 0..state.elements.len() {
            wanted_state.elements.push([3, 3]);
        }

        // search for an optimal solution
        match bfs_search(state, wanted_state) {
            Some(steps) => Ok(steps.to_string()),
            None => Err(Error::NoSolution),
        }
    }
}

// the main idea for a fast solution is to not differentiate between pairs of elements
// if element1-microchip is on floor 0, element1-generator on floor 3
// and element2-microchip is on floor 1, element2-generator on floor 2
// this state is the same as
// if element1-microchip is on floor 1, element1-generator on floor 2
// and element2-microchip is on floor 0, element2-generator on floor 3

/// Search for the minimum number of steps to get from initial_state to wanted_state using BFS.
/// Returns `None` if no solution is found
/// (there is no way to get to wanted_state from initial_state).
fn bfs_search(initial_state: State, wanted_state: State) -> Option<i32> {
    if initial_state == wanted_state {
        return Some(0);
    }

    let mut seen_states = HashSet::new();
    let mut queue = VecDeque::new();
    seen_states.insert(initial_state.clone());
    queue.push_back((initial_state, 0));

    let mut up_down_moves = Vec::with_capacity(2);
    while let Some((state, steps)) = queue.pop_front() {
        up_down_moves.clear();
        if state.elevator > 0 {
            up_down_moves.push(-1);
        }
        if state.elevator < 3 {
            up_down_moves.push(1);
        }

        for &k in &up_down_moves {
            // moving 1 element
            for i in 0..(state.elements.len() << 1) {
                if state.elements[i >> 1][i & 1] == state.elevator {
                    let mut next_state = state.clone();
                    next_state.elevator = next_state.elevator.wrapping_add_signed(k);
                    next_state.elements[i >> 1][i & 1] =
                        next_state.elements[i >> 1][i & 1].wrapping_add_signed(k);
                    next_state.elements.sort();
                    if next_state.is_valid() && !seen_states.contains(&next_state) {
                        if next_state == wanted_state {
                            return Some(steps + 1);
                        } else {
                            queue.push_back((next_state.clone(), steps + 1));
                            seen_states.insert(next_state);
                        }
                    }
                }
            }

            for i in 0..(state.elements.len() << 1) {
                if state.elements[i >> 1][i & 1] == state.elevator {
                    for j in (i + 1)..(state.elements.len() << 1) {
                        if state.elements[j >> 1][j & 1] == state.elevator {
                            let mut next_state = state.clone();
                            next_state.elevator = next_state.elevator.wrapping_add_signed(k);
                            next_state.elements[i >> 1][i & 1] =
                                next_state.elements[i >> 1][i & 1].wrapping_add_signed(k);
                            next_state.elements[j >> 1][j & 1] =
                                next_state.elements[j >> 1][j & 1].wrapping_add_signed(k);
                            next_state.elements.sort();
                            if next_state.is_valid() && !seen_states.contains(&next_state) {
                                if next_state == wanted_state {
                                    return Some(steps + 1);
                                } else {
                                    queue.push_back((next_state.clone(), steps + 1));
                                    seen_states.insert(next_state);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    elevator: u8,
    elements: SmallVec<[[u8; 2]; 10]>, // vector of elements, with [microchip, generator] (0..3 -> floor)
}
impl State {
    fn new() -> Self {
        Self {
            elevator: 0,
            elements: SmallVec::with_capacity(10),
        }
    }

    /// Generate the initial building state from input
    fn from_input(input: &str) -> Self {
        let mut element_id = HashMap::new();
        let mut floors = vec![vec![]; 4];

        let re = Regex::new(r", and|and|,").unwrap();
        for (i, floor_line) in input.lines().enumerate() {
            if floor_line.contains("nothing relevant") {
                continue;
            }
            re.split(
                floor_line
                    .trim_end_matches('.')
                    .split_once(" contains ")
                    .unwrap()
                    .1,
            )
            .for_each(|element| {
                let mut element = element.trim().trim_start_matches("a ");
                let is_generator = if element.ends_with("generator") {
                    element = element.trim_end_matches(" generator");
                    true
                } else {
                    element = element.trim_end_matches("-compatible microchip");
                    false
                };
                let len = element_id.len() as u8;
                let id = *element_id.entry(element).or_insert(len);
                floors[i].push((id, is_generator));
            });
        }

        let mut elements = Vec::with_capacity(element_id.len());
        for _ in 0..element_id.len() {
            elements.push([0, 0]);
        }
        for (i, floor) in floors.into_iter().enumerate() {
            for (id, is_generator) in floor {
                elements[id as usize][usize::from(is_generator)] = i as u8;
            }
        }

        let mut state = Self::new();
        state.elements.extend(elements);
        state.elements.sort();

        state
    }

    /// Check if the state is valid
    fn is_valid(&self) -> bool {
        for floor in 0..4 {
            // if there is no generator on the floor, it must be valid
            if self.elements.iter().any(|element| element[1] == floor) {
                for element in &self.elements {
                    if element[0] == floor && element[0] != element[1] {
                        return false;
                    }
                }
            }
        }
        true
    }
}
