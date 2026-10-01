use crate::{Error, Solution};

day!(Day03, 2017, 3, "Spiral Memory");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let wanted = input.trim().parse::<u32>().unwrap();
        let n = (((wanted as f64 - 1.0).sqrt() + 1.0) / 2.0).floor() as i32;
        let mut leftover = wanted as i32 - (2 * n - 1).pow(2) - 1;
        let mut x = n;
        let mut y = 1 - n;

        // up right corner
        if leftover > 0 {
            let diff = n - y;
            if leftover >= diff {
                y = n;
                leftover -= diff;
            } else {
                y += leftover;
                leftover = 0;
            }
        }

        // up left corner
        if leftover > 0 {
            let diff = 2 * n;
            if leftover >= diff {
                x = -n;
                leftover -= diff;
            } else {
                x -= leftover;
                leftover = 0;
            }
        }

        // down the left corner
        if leftover > 0 {
            let diff = 2 * n;
            if leftover >= diff {
                y = -n;
                leftover -= diff;
            } else {
                y -= leftover;
                leftover = 0;
            }
        }

        // down right corner
        if leftover > 0 {
            x += leftover;
        }

        Ok((x.abs() + y.abs()).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let limit: u32 = input.trim().parse().unwrap();

        let mut table: Vec<Vec<u32>> = vec![vec![1]];
        let mut coords = [0, 0];

        loop {
            // expand table
            coords[0] += if coords[0] == 0 { 1 } else { 2 };
            coords[1] += 2;
            for line in table.iter_mut() {
                line.insert(0, 0);
                line.push(0);
            }
            table.insert(0, vec![0; table[0].len()]);
            table.push(vec![0; table[0].len()]);

            // calculate new values
            let mut new_coords = coords;
            loop {
                let mut sum = 0;
                // right
                sum += if table[new_coords[0]].get(new_coords[1] + 1).is_some() {
                    table[new_coords[0]][new_coords[1] + 1]
                } else {
                    0
                };
                // down
                sum += if table.get(new_coords[0] + 1).is_some() {
                    table[new_coords[0] + 1][new_coords[1]]
                } else {
                    0
                };
                // right down
                sum += if table.get(new_coords[0] + 1).is_some()
                    && table[new_coords[0] + 1].get(new_coords[1] + 1).is_some()
                {
                    table[new_coords[0] + 1][new_coords[1] + 1]
                } else {
                    0
                };
                // up
                sum += if new_coords[0] > 0 {
                    table[new_coords[0] - 1][new_coords[1]]
                } else {
                    0
                };
                // left
                sum += if new_coords[1] > 0 {
                    table[new_coords[0]][new_coords[1] - 1]
                } else {
                    0
                };
                // left down
                sum += if new_coords[1] > 0 && table.get(new_coords[0] + 1).is_some() {
                    table[new_coords[0] + 1][new_coords[1] - 1]
                } else {
                    0
                };
                // up left
                sum += if new_coords[0] > 0 && new_coords[1] > 0 {
                    table[new_coords[0] - 1][new_coords[1] - 1]
                } else {
                    0
                };
                // up right
                sum += if new_coords[0] > 0 && table[new_coords[0]].get(new_coords[1] + 1).is_some()
                {
                    table[new_coords[0] - 1][new_coords[1] + 1]
                } else {
                    0
                };

                if sum > limit {
                    return Ok(sum.to_string());
                } else {
                    table[new_coords[0]][new_coords[1]] = sum;
                    if new_coords[1] == table[0].len() - 1 {
                        if new_coords[0] > 0 {
                            new_coords[0] -= 1;
                        } else {
                            new_coords[1] -= 1;
                        }
                    } else if new_coords[0] == 0 {
                        if new_coords[1] > 0 {
                            new_coords[1] -= 1;
                        } else {
                            new_coords[0] += 1;
                        }
                    } else if new_coords[1] == 0 {
                        if new_coords[0] < table.len() - 1 {
                            new_coords[0] += 1;
                        } else {
                            new_coords[1] += 1;
                        }
                    // we know we are at the bottom of the table
                    } else if new_coords[1] < table[0].len() - 1 {
                        new_coords[1] += 1;
                    } else {
                        new_coords[0] -= 1;
                    }

                    if new_coords == coords {
                        break;
                    }
                }
            }
        }
    }
}
