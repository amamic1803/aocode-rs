use crate::{Error, Solution};

day!(Day05, 2022, 5, "Supply Stacks");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let num_of_piles = count_piles(input);
        let mut piles: Vec<Vec<char>> = vec![];
        for _ in 0..num_of_piles {
            piles.push(vec![]);
        }
        let mut move_vec: Vec<&str>;
        for line in input.trim_end().lines() {
            if line.contains('[') {
                for (ind, character) in line.chars().enumerate() {
                    if ind % 4 == 1 && character != ' ' {
                        piles[ind / 4].insert(0, character);
                    }
                    if ind % 4 == 1 && (ind / 4) + 1 == num_of_piles {
                        break;
                    }
                }
            } else if line.contains("move") {
                move_vec = line.trim().split(' ').collect();
                for _ in 0..move_vec[1].parse::<usize>().unwrap() {
                    let character: char = piles[move_vec[3].parse::<usize>().unwrap() - 1]
                        [piles[move_vec[3].parse::<usize>().unwrap() - 1].len() - 1];
                    piles[move_vec[5].parse::<usize>().unwrap() - 1].push(character);
                    piles[move_vec[3].parse::<usize>().unwrap() - 1].pop();
                }
            }
        }
        let mut output: String = String::new();
        for pile in piles {
            output.push(pile[pile.len() - 1]);
        }
        Ok(output.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let num_of_piles = count_piles(input);
        let mut piles: Vec<Vec<char>> = vec![];
        for _ in 0..num_of_piles {
            piles.push(vec![]);
        }
        let mut move_vec: Vec<&str>;
        for line in input.trim_end().lines() {
            if line.contains('[') {
                for (ind, character) in line.chars().enumerate() {
                    if ind % 4 == 1 && character != ' ' {
                        piles[ind / 4].insert(0, character);
                    }
                    if ind % 4 == 1 && (ind / 4) + 1 == num_of_piles {
                        break;
                    }
                }
            } else if line.contains("move") {
                move_vec = line.trim().split(' ').collect();
                let insert_ind = piles[move_vec[5].parse::<usize>().unwrap() - 1].len();
                for _ in 0..move_vec[1].parse::<usize>().unwrap() {
                    let character: char = piles[move_vec[3].parse::<usize>().unwrap() - 1]
                        [piles[move_vec[3].parse::<usize>().unwrap() - 1].len() - 1];
                    piles[move_vec[5].parse::<usize>().unwrap() - 1].insert(insert_ind, character);
                    piles[move_vec[3].parse::<usize>().unwrap() - 1].pop();
                }
            }
        }
        let mut output: String = String::new();
        for pile_part in piles {
            output.push(pile_part[pile_part.len() - 1]);
        }
        Ok(output.to_string())
    }
}

fn count_piles(input: &str) -> usize {
    // loops until it finds the line with the pile numbers and reads the last number

    let mut num_of_piles: usize = 0;
    for line in input.trim().lines() {
        if !line.contains('[') {
            num_of_piles = line
                .trim()
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            break;
        }
    }
    num_of_piles
}
