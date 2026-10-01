use crate::{Error, Solution};

day!(Day05, 2015, 5, "Doesn't He Have Intern-Elves For This?");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(count_nice_strings(input, |string: &str| {
            let vowel_count = string.chars().filter(|c| "aeiou".contains(*c)).count();

            let mut last_char = ' ';
            let mut double_letter = false;
            for (i, c) in string.chars().enumerate() {
                if c == last_char && i != 0 {
                    double_letter = true;
                    break;
                }
                last_char = c;
            }

            let disallowed_substrings = string.contains("ab")
                || string.contains("cd")
                || string.contains("pq")
                || string.contains("xy");

            vowel_count >= 3 && double_letter && !disallowed_substrings
        })
        .to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(count_nice_strings(input, |string: &str| {
            let string: Vec<char> = string.chars().collect();

            let mut double_pair = false;
            if string.len() < 4 {
                return false;
            }
            'outer: for i in 0..(string.len() - 3) {
                for j in (i + 2)..(string.len() - 1) {
                    if string[i] == string[j] && string[i + 1] == string[j + 1] {
                        double_pair = true;
                        break 'outer;
                    }
                }
            }

            let mut letter_between = false;
            for i in 1..(string.len() - 1) {
                if string[i - 1] == string[i + 1] {
                    letter_between = true;
                    break;
                }
            }

            double_pair && letter_between
        })
        .to_string())
    }
}

fn count_nice_strings<F>(input: &str, nice_str: F) -> usize
where
    F: Fn(&str) -> bool,
{
    let mut nice_strings = 0;
    for line in input.trim().lines() {
        if nice_str(line.trim()) {
            nice_strings += 1;
        }
    }
    nice_strings
}
