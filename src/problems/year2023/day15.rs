use crate::{Error, Solution};

day!(Day15, 2023, 15, "Lens Library");

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .trim()
            .split(',')
            .map(calculate_hash)
            .sum::<u32>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut hashmap = vec![vec![]; 256];

        for op in input.trim().split(',') {
            update_hashmap(&mut hashmap, op);
        }

        let mut focusing_power = 0;

        for (i, box_i) in hashmap.into_iter().enumerate() {
            for (j, lenses) in box_i.into_iter().enumerate() {
                focusing_power += (i as u32 + 1) * (j as u32 + 1) * lenses.1;
            }
        }

        Ok(focusing_power.to_string())
    }
}

fn calculate_hash(input: &str) -> u32 {
    let mut current_value = 0;

    for c in input.trim().chars() {
        assert!(c.is_ascii(), "Input must be ascii");
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn update_hashmap<'a>(hashmap: &mut [Vec<(&'a str, u32)>], operation: &'a str) {
    if operation.contains('-') {
        let label = operation.trim_end_matches('-');
        let curr_box = &mut hashmap[calculate_hash(label) as usize];

        if let Some(ind) = curr_box.iter().position(|(l, _)| l == &label) {
            curr_box.remove(ind);
        }
    } else {
        let (label, val) = operation.split_once('=').unwrap();
        let val = val.parse::<u32>().unwrap();
        let curr_box = &mut hashmap[calculate_hash(label) as usize];

        if let Some(ind) = curr_box.iter().position(|(l, _)| l == &label) {
            curr_box[ind].1 = val;
        } else {
            curr_box.push((label, val));
        }
    }
}
