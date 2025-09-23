use crate::math::HEX_DIGITS;
use crate::{Error, Solution};

use md5::{Digest, Md5};
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Write;
use std::sync::LazyLock;

day!(Day14, 2016, 14, "One-Time Pad");

impl Solution for Day14 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(calculate_passwords(input, |hash_in, hash_out| {
            let mut hasher = Md5::new();
            hasher.update(hash_in);

            hash_out.clear();
            write!(hash_out, "{:x}", hasher.finalize()).unwrap();
        })
        .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(calculate_passwords(input, |hash_in, hash_out| {
            let mut hasher = Md5::new();
            let mut temp_hash = String::with_capacity(32);
            hasher.update(hash_in);
            write!(temp_hash, "{:x}", hasher.finalize_reset()).unwrap();

            for _ in 0..ADDITIONAL_HASHES {
                hasher.update(&temp_hash);
                let hash = hasher.finalize_reset();
                temp_hash.clear();
                write!(temp_hash, "{:x}", hash).unwrap();
            }

            hash_out.clear();
            hash_out.push_str(&temp_hash);
        })
        .to_string())
    }
}

const PASSWORDS_TO_FIND: u8 = 64;
const ADDITIONAL_HASHES: usize = 2016;

fn calculate_passwords<T: Fn(&str, &mut String)>(input: &str, hash_fn: T) -> u64 {
    static RE_3: LazyLock<Vec<Regex>> = LazyLock::new(|| {
        let mut re_3 = Vec::new();
        for hex_digit in HEX_DIGITS {
            re_3.push(Regex::new(&format!("{hex_digit}{{3}}")).unwrap());
        }
        re_3
    });
    static RE_5: LazyLock<Vec<Regex>> = LazyLock::new(|| {
        let mut re_5 = Vec::new();
        for hex_digit in HEX_DIGITS {
            re_5.push(Regex::new(&format!("{hex_digit}{{5}}")).unwrap());
        }
        re_5
    });

    let mut last_occurences_3: Vec<VecDeque<u64>> = Vec::with_capacity(16);
    for _ in 0..16 {
        last_occurences_3.push(VecDeque::new());
    }
    let mut hash_in = String::from(input.trim());
    let hash_in_len = hash_in.len();
    let mut hash_out = String::new();

    let mut passwords_found = Vec::new();
    let mut last_password = 0;

    let mut i = 0;
    while passwords_found.len() < PASSWORDS_TO_FIND as usize
        || last_occurences_3
            .iter()
            .filter_map(|v| v.front())
            .any(|&val| val < last_password)
    {
        hash_in.truncate(hash_in_len);
        write!(&mut hash_in, "{i}").unwrap();

        hash_fn(&hash_in, &mut hash_out);

        // clean up last_occurences
        for last_occurences in last_occurences_3.iter_mut() {
            while let Some(last_ind) = last_occurences.front() {
                if i - last_ind > 1000 {
                    last_occurences.pop_front();
                } else {
                    break;
                }
            }
        }

        for (j, re) in RE_5.iter().enumerate() {
            if re.find(&hash_out).is_some() {
                let mut processed_items = 0;
                while let Some(last_ind) = last_occurences_3[j].get(processed_items).copied() {
                    processed_items += 1;
                    passwords_found.push(last_ind);
                    if passwords_found.len() == PASSWORDS_TO_FIND as usize {
                        last_password = last_ind;
                    }
                }
            }
        }

        let mut first_j = 0;
        let mut first_match = None;
        for (j, re) in RE_3.iter().enumerate() {
            if let Some(mat) = re.find(&hash_out)
                && mat.start() < first_match.unwrap_or(usize::MAX)
            {
                first_match = Some(mat.start());
                first_j = j;
            }
        }

        if first_match.is_some() {
            last_occurences_3[first_j].push_back(i);
        }

        i += 1;
    }

    passwords_found.sort();
    passwords_found.truncate(PASSWORDS_TO_FIND as usize);
    passwords_found[PASSWORDS_TO_FIND as usize - 1]
}
