use crate::{Error, Solution};

day!(Day06, 2020, 6, "Custom Customs");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut group_answers = [false; 26];
        let mut counts_sum = 0;

        for line in input.lines() {
            if line.is_empty() {
                counts_sum += group_answers.iter().filter(|&&x| x).count();
                group_answers = [false; 26];
            } else {
                for c in line.chars() {
                    group_answers[(c as u32 - 'a' as u32) as usize] = true;
                }
            }
        }
        counts_sum += group_answers.iter().filter(|&&x| x).count();

        Ok(counts_sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut group_answers = [0; 26];
        let mut group_size = 0;
        let mut counts_sum = 0;

        for line in input.lines() {
            if line.is_empty() {
                counts_sum += group_answers.iter().filter(|&&x| x == group_size).count();
                group_answers = [0; 26];
                group_size = 0;
            } else {
                for c in line.chars() {
                    group_answers[(c as u32 - 'a' as u32) as usize] += 1;
                }
                group_size += 1;
            }
        }
        counts_sum += group_answers.iter().filter(|&&x| x == group_size).count();

        Ok(counts_sum.to_string())
    }
}
