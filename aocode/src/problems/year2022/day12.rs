use crate::{Error, Solution};
use std::collections::VecDeque;

day!(Day12, 2022, 12, "Hill Climbing Algorithm");

impl Solution for Day12 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (height_map, loc_start, loc_end) = parse_input(input);
        Ok(dijkstra_1(&height_map, loc_start, loc_end).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (height_map, _, loc_end) = parse_input(input);
        Ok(dijkstra_2(&height_map, loc_end, 0).to_string())
    }
}

fn dijkstra_1(height_map: &[Vec<u32>], loc_start: [usize; 2], loc_end: [usize; 2]) -> u32 {
    //! Returns the lowest number of steps required to reach the end location
    //! Or 0 if there is no path
    //! Solves Part 1

    let mut visit_map: Vec<Vec<bool>> = vec![];
    let mut distance_map: Vec<Vec<u32>> = vec![];
    let mut queue: VecDeque<[usize; 2]> = VecDeque::new();

    for _ in 0..height_map.len() {
        visit_map.push(vec![false; height_map[0].len()]);
        distance_map.push(vec![0; height_map[0].len()]);
    }

    visit_map[loc_start[0]][loc_start[1]] = true;
    queue.push_back(loc_start);

    let mut next_location: Vec<[usize; 2]> = Vec::with_capacity(4);
    while !queue.is_empty() {
        let step_moves = queue.len();
        for _ in 0..step_moves {
            let curr_loc = queue.pop_front().unwrap();

            if curr_loc == loc_end {
                return distance_map[curr_loc[0]][curr_loc[1]];
            }

            next_location.clear();

            if curr_loc[0] != 0 {
                next_location.push([curr_loc[0] - 1, curr_loc[1]]);
            }
            if curr_loc[0] != height_map.len() - 1 {
                next_location.push([curr_loc[0] + 1, curr_loc[1]]);
            }
            if curr_loc[1] != 0 {
                next_location.push([curr_loc[0], curr_loc[1] - 1]);
            }
            if curr_loc[1] != height_map[0].len() - 1 {
                next_location.push([curr_loc[0], curr_loc[1] + 1]);
            }

            next_location.retain(|x| {
                let not_visited = !visit_map[x[0]][x[1]];
                let possible_height =
                    if height_map[x[0]][x[1]] > height_map[curr_loc[0]][curr_loc[1]] {
                        height_map[x[0]][x[1]] - height_map[curr_loc[0]][curr_loc[1]] <= 1
                    } else {
                        true
                    };
                not_visited && possible_height
            });

            for next_loc in &next_location {
                visit_map[next_loc[0]][next_loc[1]] = true;
                distance_map[next_loc[0]][next_loc[1]] = distance_map[curr_loc[0]][curr_loc[1]] + 1;
                queue.push_back(*next_loc);
            }
        }
    }
    0
}

fn dijkstra_2(height_map: &[Vec<u32>], loc_end: [usize; 2], lowest_level: u32) -> u32 {
    //! Returns the lowest number of steps required to reach the end location from lowest level
    //! Or 0 if there is no path
    //! Solves Part 2

    let mut visit_map: Vec<Vec<bool>> = vec![];
    let mut distance_map: Vec<Vec<u32>> = vec![];
    let mut queue: VecDeque<[usize; 2]> = VecDeque::new();

    for _ in 0..height_map.len() {
        visit_map.push(vec![false; height_map[0].len()]);
        distance_map.push(vec![0; height_map[0].len()]);
    }

    visit_map[loc_end[0]][loc_end[1]] = true;
    queue.push_back(loc_end);

    let mut next_location: Vec<[usize; 2]> = Vec::with_capacity(4);
    while !queue.is_empty() {
        let step_moves = queue.len();
        for _ in 0..step_moves {
            let curr_loc = queue.pop_front().unwrap();

            if height_map[curr_loc[0]][curr_loc[1]] == lowest_level {
                return distance_map[curr_loc[0]][curr_loc[1]];
            }

            next_location.clear();

            if curr_loc[0] != 0 {
                next_location.push([curr_loc[0] - 1, curr_loc[1]]);
            }
            if curr_loc[0] != height_map.len() - 1 {
                next_location.push([curr_loc[0] + 1, curr_loc[1]]);
            }
            if curr_loc[1] != 0 {
                next_location.push([curr_loc[0], curr_loc[1] - 1]);
            }
            if curr_loc[1] != height_map[0].len() - 1 {
                next_location.push([curr_loc[0], curr_loc[1] + 1]);
            }

            next_location.retain(|x| {
                let not_visited = !visit_map[x[0]][x[1]];
                let possible_height =
                    if height_map[x[0]][x[1]] < height_map[curr_loc[0]][curr_loc[1]] {
                        height_map[curr_loc[0]][curr_loc[1]] - height_map[x[0]][x[1]] <= 1
                    } else {
                        true
                    };
                not_visited && possible_height
            });

            for next_loc in &next_location {
                visit_map[next_loc[0]][next_loc[1]] = true;
                distance_map[next_loc[0]][next_loc[1]] = distance_map[curr_loc[0]][curr_loc[1]] + 1;
                queue.push_back(*next_loc);
            }
        }
    }
    0
}

fn char_value(mut character: char) -> u32 {
    match character {
        'S' => character = 'a',
        'E' => character = 'z',
        _ => {}
    }
    character as u32 - 'a' as u32
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, [usize; 2], [usize; 2]) {
    let mut height_map: Vec<Vec<u32>> = vec![];

    let mut loc_start: [usize; 2] = [0; 2];
    let mut loc_end: [usize; 2] = [0; 2];

    for (curr_row, line) in input.trim().lines().enumerate() {
        let mut current_row: Vec<u32> = vec![];
        for (curr_col, x) in line.chars().enumerate() {
            if x == 'S' {
                loc_start = [curr_row, curr_col];
            } else if x == 'E' {
                loc_end = [curr_row, curr_col];
            }
            current_row.push(char_value(x));
        }
        height_map.push(current_row);
    }

    (height_map, loc_start, loc_end)
}
