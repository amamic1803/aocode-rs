use crate::{Error, Solution};
use md5::{Digest, Md5};

day!(Day17, 2016, 17, "Two Steps Forward");

impl Solution for Day17 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Room::new(input).shortest_exit_path())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Room::new(input)
            .longest_exit_path()
            .chars()
            .count()
            .to_string())
    }
}

struct Room {
    passcode: String,
}
impl Room {
    fn new(input: &str) -> Self {
        Self {
            passcode: input.trim().to_string(),
        }
    }

    fn shortest_exit_path(&mut self) -> String {
        let mut shortest_path_length = usize::MAX;
        let mut shortest_path_hash = String::new();
        let mut path_hash = self.passcode.clone();

        fn recursive_search(
            depth: usize,
            location: (usize, usize),
            path_hash: &mut String,
            shortest_path_length: &mut usize,
            shortest_path_hash: &mut String,
        ) {
            // depth == path length
            if depth < *shortest_path_length {
                if location == (3, 3) {
                    *shortest_path_length = depth;
                    shortest_path_hash.clear();
                    shortest_path_hash.push_str(path_hash);
                } else {
                    let mut hasher = Md5::new();
                    hasher.update(&path_hash);
                    let mut hex_iter = hasher
                        .finalize()
                        .into_iter()
                        .flat_map(|byte| [byte / 16, byte % 16])
                        .map(|num| num >= 11); // b,c,d,e,f == 11,12,13,14,15 => open doors

                    // up
                    if hex_iter.next().unwrap() && location.0 > 0 {
                        path_hash.push('U');
                        recursive_search(
                            depth + 1,
                            (location.0 - 1, location.1),
                            path_hash,
                            shortest_path_length,
                            shortest_path_hash,
                        );
                        path_hash.pop();
                    }

                    // down
                    if hex_iter.next().unwrap() && location.0 < 3 {
                        path_hash.push('D');
                        recursive_search(
                            depth + 1,
                            (location.0 + 1, location.1),
                            path_hash,
                            shortest_path_length,
                            shortest_path_hash,
                        );
                        path_hash.pop();
                    }

                    // left
                    if hex_iter.next().unwrap() && location.1 > 0 {
                        path_hash.push('L');
                        recursive_search(
                            depth + 1,
                            (location.0, location.1 - 1),
                            path_hash,
                            shortest_path_length,
                            shortest_path_hash,
                        );
                        path_hash.pop();
                    }

                    // right
                    if hex_iter.next().unwrap() && location.1 < 3 {
                        path_hash.push('R');
                        recursive_search(
                            depth + 1,
                            (location.0, location.1 + 1),
                            path_hash,
                            shortest_path_length,
                            shortest_path_hash,
                        );
                        path_hash.pop();
                    }
                }
            }
        }

        recursive_search(
            0,
            (0, 0),
            &mut path_hash,
            &mut shortest_path_length,
            &mut shortest_path_hash,
        );

        if shortest_path_length == usize::MAX {
            panic!("No exit path found!");
        } else {
            shortest_path_hash
                .trim_start_matches(&self.passcode)
                .to_string()
        }
    }

    fn longest_exit_path(&mut self) -> String {
        let mut longest_path_length = 0;
        let mut longest_path_hash = String::new();
        let mut path_hash = self.passcode.clone();

        fn recursive_search(
            depth: usize,
            location: (usize, usize),
            path_hash: &mut String,
            longest_path_length: &mut usize,
            longest_path_hash: &mut String,
        ) {
            // depth == path length

            if location == (3, 3) {
                if depth > *longest_path_length {
                    *longest_path_length = depth;
                    longest_path_hash.clear();
                    longest_path_hash.push_str(path_hash);
                }
            } else {
                let mut hasher = Md5::new();
                hasher.update(&path_hash);
                let mut hex_iter = hasher
                    .finalize()
                    .into_iter()
                    .flat_map(|byte| [byte / 16, byte % 16])
                    .map(|num| num >= 11); // b,c,d,e,f == 11,12,13,14,15 => open doors

                // up
                if hex_iter.next().unwrap() && location.0 > 0 {
                    path_hash.push('U');
                    recursive_search(
                        depth + 1,
                        (location.0 - 1, location.1),
                        path_hash,
                        longest_path_length,
                        longest_path_hash,
                    );
                    path_hash.pop();
                }

                // down
                if hex_iter.next().unwrap() && location.0 < 3 {
                    path_hash.push('D');
                    recursive_search(
                        depth + 1,
                        (location.0 + 1, location.1),
                        path_hash,
                        longest_path_length,
                        longest_path_hash,
                    );
                    path_hash.pop();
                }

                // left
                if hex_iter.next().unwrap() && location.1 > 0 {
                    path_hash.push('L');
                    recursive_search(
                        depth + 1,
                        (location.0, location.1 - 1),
                        path_hash,
                        longest_path_length,
                        longest_path_hash,
                    );
                    path_hash.pop();
                }

                // right
                if hex_iter.next().unwrap() && location.1 < 3 {
                    path_hash.push('R');
                    recursive_search(
                        depth + 1,
                        (location.0, location.1 + 1),
                        path_hash,
                        longest_path_length,
                        longest_path_hash,
                    );
                    path_hash.pop();
                }
            }
        }

        recursive_search(
            0,
            (0, 0),
            &mut path_hash,
            &mut longest_path_length,
            &mut longest_path_hash,
        );

        if longest_path_length == 0 {
            panic!("No exit path found!");
        } else {
            longest_path_hash
                .trim_start_matches(&self.passcode)
                .to_string()
        }
    }
}
