use crate::{Error, Solution};

day!(Day01, 2023, 1, "Trebuchet?!");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;
        let lines = parse_input(input);
        for line in lines {
            let mut digits_iterator = line.chars().filter_map(|c| c.to_digit(10));
            let mut digits_iterator_back = digits_iterator.clone();
            sum += digits_iterator.next().unwrap() as usize * 10
                + digits_iterator_back.next_back().unwrap() as usize;
        }
        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;
        let lines = parse_input(input);

        for line in lines {
            let mut digits_iterator = line
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
                .map(|(pos, c)| (pos, c.to_digit(10).unwrap()));

            let (front_digit_pos, mut front_digit) = digits_iterator.next().unwrap();
            let (mut back_digit_pos, mut back_digit) = (front_digit_pos, front_digit);
            for digit in digits_iterator {
                (back_digit_pos, back_digit) = digit;
            }

            let mut letter_digits_front: [Option<usize>; 9] = [None; 9];
            let mut letter_digits_back: [Option<usize>; 9] = [None; 9];
            for digit in SPELLED_DIGITS.iter().enumerate() {
                letter_digits_front[digit.0] = line.find(digit.1);
                letter_digits_back[digit.0] = line.rfind(digit.1);
            }

            let front_spelled_digit = letter_digits_front
                .iter()
                .enumerate()
                .filter(|(_, pos)| pos.is_some())
                .map(|(digit, pos)| (digit + 1, pos.unwrap()))
                .min_by_key(|(_, pos)| *pos);

            let back_spelled_digit = letter_digits_back
                .iter()
                .enumerate()
                .filter(|(_, pos)| pos.is_some())
                .map(|(digit, pos)| (digit + 1, pos.unwrap()))
                .max_by_key(|(_, pos)| *pos);

            if let Some((digit, i)) = front_spelled_digit
                && i < front_digit_pos
            {
                front_digit = digit as u32;
            }

            if let Some((digit, i)) = back_spelled_digit
                && i > back_digit_pos
            {
                back_digit = digit as u32;
            }

            sum += front_digit as usize * 10 + back_digit as usize;
        }

        Ok(sum.to_string())
    }
}

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().map(|line| line.trim()).collect()
}
