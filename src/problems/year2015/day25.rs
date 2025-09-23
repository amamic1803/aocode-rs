use crate::{Error, Solution};

day!(Day25, 2015, 25, "Let It Snow");

impl Solution for Day25 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (row, col) = parse_input(input);
        let position_in_seq = seq_n(row, col);
        Ok(calculate_code(position_in_seq).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let _ = input; // suppress unused variable warning
        Ok(String::from("Advent of Code 2015 solved!"))
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut line_iter = input.trim().lines().next().unwrap().split_whitespace();

    let mut element = line_iter.next().unwrap();
    while element != "row" {
        element = line_iter.next().unwrap();
    }
    let row = line_iter
        .next()
        .unwrap()
        .trim_end_matches([',', '.'])
        .parse::<usize>()
        .unwrap();

    while element != "column" {
        element = line_iter.next().unwrap();
    }
    let col = line_iter
        .next()
        .unwrap()
        .trim_end_matches([',', '.'])
        .parse::<usize>()
        .unwrap();

    // (row, column)
    (row, col)
}

fn seq_n(row: usize, col: usize) -> usize {
    //! Returns which position in the sequence the code is at regarding the row and column

    // we find which diagonal the code is on by similarity of triangles
    let diagonal = row + col - 1;

    // we find sum all diagonals before this one and then just count how many numbers are in this diagonal before wanted one
    // the number of elements of each diagonal is the diagonal number, so the sum of all diagonals before this one is the sum of all numbers from 1 to diagonal (exclusive)
    // we get the formula for finding code position
    // (diagonal - 1)(diagonal) / 2 + col

    ((diagonal - 1) * diagonal) / 2 + col
}

fn calculate_code(position: usize) -> usize {
    let mut current_code = 20151125;

    for _ in 0..(position - 1) {
        current_code = (current_code * 252533) % 33554393;
    }

    current_code
}
