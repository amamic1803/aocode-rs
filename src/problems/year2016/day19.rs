use crate::{Error, Solution};

day!(Day19, 2016, 19, "An Elephant Named Joseph");

impl Solution for Day19 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let elves = match input.trim().parse() {
            Ok(num) => {
                assert!(num > 0, "Number of elves must be greater than 0");
                num
            }
            Err(_) => return Err(Error::NoSolution),
        };

        let mut circle = Vec::with_capacity(elves);
        for i in 0..(elves - 1) {
            circle.push(i + 1);
        }
        circle.push(0);

        let mut current_elf = 0;
        loop {
            let next_elf = circle[current_elf];
            let new_next_elf = circle[next_elf];
            if new_next_elf == current_elf {
                return Ok((current_elf + 1).to_string());
            } else {
                circle[current_elf] = new_next_elf;
                current_elf = new_next_elf;
            }
        }
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let elves = match input.trim().parse() {
            Ok(num) => {
                assert!(num > 0, "Number of elves must be greater than 0");
                num
            }
            Err(_) => return Err(Error::NoSolution),
        };

        let mut circle = Vec::with_capacity(elves);
        for i in 0..(elves - 1) {
            circle.push(i + 1);
        }
        circle.push(0);

        let mut circle_len = elves;
        let mut opposite_elf = elves / 2;
        let mut opposite_elf_prev = opposite_elf - 1;
        while circle_len > 1 {
            opposite_elf = circle[opposite_elf];
            circle[opposite_elf_prev] = opposite_elf;
            if circle_len % 2 != 0 {
                (opposite_elf, opposite_elf_prev) = (circle[opposite_elf], opposite_elf);
            }
            circle_len -= 1;
        }

        Ok((opposite_elf + 1).to_string())
    }
}
