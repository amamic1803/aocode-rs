use crate::{Error, Solution};

day!(Day03, 2021, 3, "Binary Diagnostic");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let data = parse_input(input);
        let mut gamma: u32 = 0;

        // for each column, count the number of 1s and 0s
        // only one variable (count) is used
        // for true it is incremented, for false it is decremented
        // if it is positive, the bit is 1, otherwise it is 0
        for i in 0..data[0].len() {
            let mut count = 0;
            for data_row in &data {
                if data_row[i] {
                    count += 1;
                } else {
                    count -= 1;
                }
            }

            // add the bit to the gamma value (to the right)
            gamma <<= 1;
            if count > 0 {
                gamma |= 1;
            }
        }

        // gamma bits need to be inverted
        // simple bitwise NOT does not work (since only the number of bits used should be inverted, but NOT inverts all bits)
        // instead, the gamma value is XORed with a number with the same number of bits as gamma, but all 1ss
        // first, create the inverse gamma value
        let mut inverse_gamma: u32 = 0;
        for _ in 0..data[0].len() {
            inverse_gamma <<= 1;
            inverse_gamma |= 1;
        }

        Ok((gamma * (gamma ^ inverse_gamma)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let data = parse_input(input);

        // oxygen
        let mut data_oxygen = data.clone();
        let mut i = 0;
        while data_oxygen.len() != 1 {
            let mut count = 0;
            for data_line in &data_oxygen {
                if data_line[i] {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            let pattern = count >= 0;
            data_oxygen.retain(|data_line| data_line[i] == pattern);
            i += 1;
        }
        let mut oxygen_value: u32 = 0;
        for bit in data_oxygen[0].iter() {
            oxygen_value <<= 1;
            if *bit {
                oxygen_value |= 1;
            }
        }

        // carbon dioxide
        let mut data_carbon = data;
        let mut i = 0;
        while data_carbon.len() != 1 {
            let mut count = 0;
            for data_line in &data_carbon {
                if data_line[i] {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            let pattern = count < 0;
            data_carbon.retain(|data_line| data_line[i] == pattern);
            i += 1;
        }
        let mut carbon_value: u32 = 0;
        for bit in data_carbon[0].iter() {
            carbon_value <<= 1;
            if *bit {
                carbon_value |= 1;
            }
        }

        Ok((oxygen_value * carbon_value).to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    // store each line as a vector of bools
    // true is a 1, false is a 0

    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '1' => true,
                    '0' => false,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}
