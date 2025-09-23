use crate::{Error, Solution};
use std::cmp::Ordering;
use std::iter;

day!(Day09, 2024, 9, "Disk Fragmenter");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(compress1(input)
            .enumerate()
            .map(|(i, block)| i as u64 * block as u64)
            .sum::<u64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(compress2(input)
            .enumerate()
            .map(|(i, block)| i as u64 * block.unwrap_or(0) as u64)
            .sum::<u64>()
            .to_string())
    }
}

fn compress1(disc: &str) -> impl Iterator<Item = u32> + use<> {
    let mut disc = disc
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    if disc.len() % 2 == 0 {
        disc.pop();
    }
    let mut i = 0;
    iter::from_fn(move || {
        loop {
            if i >= disc.len() {
                return None;
            }

            if disc[i] != 0 {
                disc[i] -= 1;
                return Some(i as u32 >> 1);
            }

            let last_ind = disc.len() - 1;
            if disc[last_ind] != 0 {
                if disc[i + 1] != 0 {
                    disc[i + 1] -= 1;
                    disc[last_ind] -= 1;
                    return Some(last_ind as u32 >> 1);
                } else {
                    i += 2;
                }
            } else {
                disc.pop();
                disc.pop();
            }
        }
    })
}

fn compress2(disc: &str) -> impl Iterator<Item = Option<u32>> + use<> {
    let mut disc = disc
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    if disc.len() % 2 == 0 {
        disc.pop();
    }
    let mut files = Vec::with_capacity(disc.len() / 2 + 1);
    let mut free_space = Vec::with_capacity(disc.len() / 2);
    let mut position = 0;
    let mut file_index = 0;
    for (i, item) in disc.into_iter().enumerate() {
        if item == 0 {
            continue;
        }
        if i % 2 == 0 {
            // file
            // (disc_position_index, file_order_index, file_size)
            files.push((position, file_index, item));
            file_index += 1;
        } else {
            // free space
            // (disc_position_index, free_space_size)
            free_space.push((position, item));
        }
        position += item as u32;
    }

    // positions of the next free block of at least [index] + 1 size
    let mut fb_next = [0; 9];
    fn fb_next_update_index(index: usize, fb_next: &mut [usize; 9], free_space: &[(u32, u8)]) {
        let start = fb_next[index];
        fb_next[index] = usize::MAX;
        for (i, free_block) in free_space[start..].iter().enumerate() {
            if free_block.1 as usize > index {
                fb_next[index] = start + i;
                break;
            }
        }
    }
    for i in 0..9 {
        fb_next_update_index(i, &mut fb_next, &free_space);
    }

    for file in files.iter_mut().rev() {
        // if the first position of the free block
        // that can fit the file is lower than the current position,
        // move the file to that position and update free_space and fb_next
        let i = fb_next[file.2 as usize - 1];
        if i == usize::MAX {
            continue;
        }
        if free_space[i].0 < file.0 {
            file.0 = free_space[i].0;
            free_space[i].0 += file.2 as u32;
            free_space[i].1 -= file.2;
            // update fb_next
            for i in (free_space[i].1 as usize)..9 {
                fb_next_update_index(i, &mut fb_next, &free_space);
            }
        }
    }
    files.sort_by_key(|file| file.0);

    let mut i = 0; // disc position
    let mut j = 0; // index of the file based on the new order
    // temp variable to count the number of times the file id is repeated
    // (file size)
    let mut k = 0;
    iter::from_fn(move || {
        loop {
            // if there are no more files, end the iterator
            if j >= files.len() {
                return None;
            }

            match i.cmp(&files[j].0) {
                Ordering::Less => {
                    // if a disc position is lower than the next file we are looking at,
                    // return None (means '.')
                    i += 1;
                    return Some(None);
                }
                Ordering::Equal => {
                    // if a disc position is equal to the position of the next file,
                    // it means it has just been reached, and counter k needs to be set up
                    k = files[j].2;
                    i += 1;
                    continue;
                }
                Ordering::Greater => {
                    // here we use k to emit file index the required number of times
                    // (k, or file size)
                    if k > 0 {
                        k -= 1;
                        return Some(Some(files[j].1));
                    } else {
                        i = files[j].0 + files[j].2 as u32;
                        j += 1;
                        continue;
                    }
                }
            }
        }
    })
}
