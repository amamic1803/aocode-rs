use crate::{Error, Solution};

day!(Day05, 2020, 5, "Binary Boarding");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .lines()
            .map(BoardingPass::new)
            .map(|b_pass| b_pass.seat_id())
            .max()
            .unwrap()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut taken_seats = input
            .lines()
            .map(BoardingPass::new)
            .map(|b_pass| b_pass.seat_id())
            .collect::<Vec<_>>();
        taken_seats.sort();

        for i in 0..(taken_seats.len() - 1) {
            if taken_seats[i + 1] - taken_seats[i] == 2 {
                return Ok((taken_seats[i] + 1).to_string());
            }
        }

        Err(Error::NoSolution)
    }
}

struct BoardingPass {
    row: u8,
    col: u8,
}
impl BoardingPass {
    fn new(position: &str) -> Self {
        let (row_ins, col_ins) = position.split_at(7);

        let mut row_range = 0..128;
        let mut col_range = 0..8;

        for c in row_ins.chars() {
            match c {
                'F' => row_range.end = row_range.start + (row_range.end - row_range.start) / 2,
                'B' => row_range.start += (row_range.end - row_range.start) / 2,
                _ => panic!("Invalid character in row instruction"),
            }
        }

        for c in col_ins.chars() {
            match c {
                'L' => col_range.end = col_range.start + (col_range.end - col_range.start) / 2,
                'R' => col_range.start += (col_range.end - col_range.start) / 2,
                _ => panic!("Invalid character in column instruction"),
            }
        }

        assert_eq!(row_range.len(), 1);
        assert_eq!(col_range.len(), 1);

        Self {
            row: row_range.start,
            col: col_range.start,
        }
    }
    fn seat_id(&self) -> u16 {
        self.row as u16 * 8 + self.col as u16
    }
}
