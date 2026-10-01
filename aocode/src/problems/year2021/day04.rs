use crate::{Error, Solution};

day!(Day04, 2021, 4, "Giant Squid");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (draws, mut boards) = parse_input(input);

        for draw in draws {
            // Mark the number on the boards
            for board in &mut boards {
                board.mark_number(draw);
            }

            // Check if any board has a bingo
            let mut bingo_board: Option<usize> = None;
            'outer: for board in boards.iter().enumerate() {
                if board.1.bingo() {
                    bingo_board = Some(board.0);
                    break 'outer;
                }
            }

            if let Some(board) = bingo_board {
                return Ok((draw * boards[board].sum_unmarked()).to_string());
            }
        }

        Err(Error::NoSolution)
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (draws, mut boards) = parse_input(input);

        // Mark the numbers on the boards
        for draw in draws {
            for board in &mut boards {
                board.mark_number(draw);
            }

            // if there are multiple boards, remove the ones with bingo
            if boards.len() > 1 {
                boards.retain(|board| !board.bingo());
            }

            // if there is a single board left and it has a bingo, return the result
            if boards.len() == 1 && boards[0].bingo() {
                return Ok((draw * boards[0].sum_unmarked()).to_string());
            }
        }

        Err(Error::NoSolution)
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Board>) {
    let mut draws = Vec::new();
    let mut boards = Vec::new();

    let mut lines = input.trim().lines();
    lines
        .next()
        .unwrap()
        .split(',')
        .for_each(|n| draws.push(n.parse::<u64>().unwrap()));
    lines.next();

    let mut board = Board::new(Vec::new());
    for line in lines {
        if line.is_empty() {
            boards.push(board);
            board = Board::new(Vec::new());
        } else {
            let mut row = Vec::new();
            for n in line.split_whitespace() {
                row.push((n.parse::<u64>().unwrap(), false));
            }
            board.values.push(row);
        }
    }

    (draws, boards)
}

// (number, is_marked)

struct Board {
    values: Vec<Vec<(u64, bool)>>,
}
impl Board {
    fn new(values: Vec<Vec<(u64, bool)>>) -> Self {
        Self { values }
    }

    fn mark_number(&mut self, n: u64) {
        for row in &mut self.values {
            for col in row {
                if col.0 == n {
                    col.1 = true;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        for row in &self.values {
            if row.iter().all(|col| col.1) {
                return true;
            }
        }

        for col in 0..self.values[0].len() {
            if self.values.iter().all(|row| row[col].1) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u64 {
        let mut sum = 0;
        for row in &self.values {
            for col in row {
                if !col.1 {
                    sum += col.0;
                }
            }
        }
        sum
    }
}
