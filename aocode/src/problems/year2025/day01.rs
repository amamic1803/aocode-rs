use crate::{Error, Solution};

day!(Day01, 2025, 1, "Secret Entrance");

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut count = 0;
        let mut value = 50;
        for rotation in input.lines() {
            let direction = &rotation[0..1];
            let amount: i32 = rotation[1..].parse().unwrap();
            match direction {
                "L" => value = (value - amount).rem_euclid(100),
                "R" => value = (value + amount).rem_euclid(100),
                _ => panic!("Invalid direction"),
            }
            if value == 0 {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut count = 0;
        let mut value = 50;
        for rotation in input.lines() {
            let direction = &rotation[0..1];
            let amount: i32 = rotation[1..].parse().unwrap();
            match direction {
                "L" => {
                    let mut new_value = value - amount;
                    if new_value == 0 {
                        count += 1;
                    } else if new_value < 0 {
                        if value != 0 {
                            count += 1;
                        }
                        count += new_value / -100;
                        new_value = new_value.rem_euclid(100);
                    }
                    value = new_value;
                },
                "R" => {
                    value += amount;
                    count += value / 100;
                    value %= 100;
                },
                _ => panic!("Invalid direction"),
            }
        }
        Ok(count.to_string())
    }
}
