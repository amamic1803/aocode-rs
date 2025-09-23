use crate::{Error, Solution};

day!(Day25, 2022, 25, "Full of Hot Air");

impl Solution for Day25 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut sum: i64 = 0;
        for line in input.trim().split('\n') {
            sum += snafu_2_dec(line);
        }
        Ok(dec_2_snafu(sum))
    }

    fn part2(&self, _input: &str) -> Result<String, Error> {
        Ok(String::from("Advent of Code 2022 solved!"))
    }
}

fn snafu_2_dec(num: &str) -> i64 {
    let mut curr_place_val: i64 = 1;
    let mut resulting_dec: i64 = 0;
    for place in num.chars().rev() {
        match place {
            '-' => resulting_dec -= curr_place_val,
            '=' => resulting_dec -= 2 * curr_place_val,
            '0' => (),
            '1' => resulting_dec += curr_place_val,
            '2' => resulting_dec += 2 * curr_place_val,
            _ => panic!(),
        }
        curr_place_val *= 5;
    }
    resulting_dec
}

fn dec_2_snafu(mut num: i64) -> String {
    let mut result: String = String::new();
    let mut working_place: i64 = 5_i64.pow((((num + 1) as f64).log(5.0).ceil() as u32) - 1);

    num *= -1;
    while num != 0 {
        let mut smallest_diff = num.abs();
        let mut smallest_diff_num = 0;
        for x in -2..3 {
            if x != 0 && smallest_diff > (x * working_place + num).abs() {
                smallest_diff = (x * working_place + num).abs();
                smallest_diff_num = x;
            }
        }

        match smallest_diff_num {
            -2 => result.push('='),
            -1 => result.push('-'),
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),
            _ => panic!(),
        }

        num += smallest_diff_num * working_place;
        working_place /= 5;
    }
    while working_place >= 1 {
        result.push('0');
        working_place /= 5;
    }

    result
}
