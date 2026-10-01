use crate::{Error, Solution};

day!(Day05, 2018, 5, "Alchemical Reduction");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut polymer = input.trim().chars().collect::<Vec<_>>();
        react(&mut polymer);
        Ok(polymer.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let polymer = input.trim().chars().collect::<Vec<_>>();
        let mut minimum = usize::MAX;

        for letter in 'a'..='z' {
            let mut removed_polymer = polymer.clone();
            let removed_letters = [letter, letter.to_ascii_uppercase()];
            removed_polymer.retain(|c| !removed_letters.contains(c));
            react(&mut removed_polymer);
            minimum = minimum.min(removed_polymer.len());
        }

        Ok(minimum.to_string())
    }
}

/// Fully react polymer
fn react(polymer: &mut Vec<char>) {
    loop {
        let mut i = 0; // read ptr
        let mut j = 0; // write ptr

        let limit = polymer.len() - 1;
        while i < limit {
            if (polymer[i].is_ascii_lowercase()
                && polymer[i + 1] == polymer[i].to_ascii_uppercase())
                || (polymer[i].is_ascii_uppercase()
                    && polymer[i + 1] == polymer[i].to_ascii_lowercase())
            {
                i += 2;
            } else {
                polymer[j] = polymer[i];
                i += 1;
                j += 1;
            }
        }

        if i == limit {
            polymer[j] = polymer[i];
            i += 1;
            j += 1;
        }

        // remove excess
        polymer.truncate(j);

        if i == j {
            break;
        } // there was no change
    }
}
