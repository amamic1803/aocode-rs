use crate::{Error, Solution};

day!(Day09, 2022, 9, "Rope Bridge");

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut head_pos: [i64; 2] = [0; 2];
        let mut tail_pos: [i64; 2] = [0; 2];
        let mut visited: Vec<[i64; 2]> = vec![tail_pos];

        for command in input.trim().lines() {
            let mut splitt = command.split(' ');
            let side = splitt.next().unwrap();
            let mut steps: i64 = splitt.next().unwrap().parse::<i64>().unwrap();
            while steps != 0 {
                match side {
                    "L" => {
                        head_pos[0] -= 1;
                        if (tail_pos[0] - head_pos[0]).abs() > 1 {
                            tail_pos[0] -= 1;
                            tail_pos[1] = head_pos[1];
                        }
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos);
                        }
                    }
                    "R" => {
                        head_pos[0] += 1;
                        if (tail_pos[0] - head_pos[0]).abs() > 1 {
                            tail_pos[0] += 1;
                            tail_pos[1] = head_pos[1];
                        }
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos);
                        }
                    }
                    "U" => {
                        head_pos[1] += 1;
                        if (tail_pos[1] - head_pos[1]).abs() > 1 {
                            tail_pos[1] += 1;
                            tail_pos[0] = head_pos[0];
                        }
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos);
                        }
                    }
                    "D" => {
                        head_pos[1] -= 1;
                        if (tail_pos[1] - head_pos[1]).abs() > 1 {
                            tail_pos[1] -= 1;
                            tail_pos[0] = head_pos[0];
                        }
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos);
                        }
                    }
                    _ => panic!(),
                }
                steps -= 1;
            }
        }
        Ok(visited.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut knot_pos: [[i64; 2]; 10] = [[0; 2]; 10];
        let mut visited: Vec<[i64; 2]> = vec![knot_pos[0]];
        for command in input.trim().lines() {
            let mut splitt = command.split(' ');
            let side = splitt.next().unwrap();
            let mut steps: i64 = splitt.next().unwrap().parse::<i64>().unwrap();
            while steps != 0 {
                let mut temp_pos = knot_pos[0];
                match side {
                    "L" => knot_pos[0][0] -= 1,
                    "R" => knot_pos[0][0] += 1,
                    "U" => knot_pos[0][1] += 1,
                    "D" => knot_pos[0][1] -= 1,
                    _ => panic!(),
                }
                for i in 1..10 {
                    if ((knot_pos[i - 1][0] - knot_pos[i][0]).abs() > 1)
                        || ((knot_pos[i - 1][1] - knot_pos[i][1]).abs() > 1)
                    {
                        let temp2_pos = knot_pos[i];

                        if (((knot_pos[i - 1][0] - knot_pos[i][0]).abs() > 1)
                            && ((temp_pos[0] - knot_pos[i][0]).abs() == 1)
                            && ((temp_pos[1] - knot_pos[i][1]).abs() == 0))
                            || (((knot_pos[i - 1][1] - knot_pos[i][1]).abs() > 1)
                                && ((temp_pos[1] - knot_pos[i][1]).abs() == 1)
                                && ((temp_pos[0] - knot_pos[i][0]).abs() == 0))
                        {
                            knot_pos[i][0] += knot_pos[i - 1][0] - temp_pos[0];
                            knot_pos[i][1] += knot_pos[i - 1][1] - temp_pos[1];
                        } else if ((knot_pos[i][0] - knot_pos[i - 1][0]).abs() > 1)
                            && ((knot_pos[i][1] - knot_pos[i - 1][1]).abs() > 1)
                        {
                            if knot_pos[i][0] - knot_pos[i - 1][0] > 0 {
                                knot_pos[i][0] -= 1;
                            } else {
                                knot_pos[i][0] += 1;
                            }
                            if knot_pos[i][1] - knot_pos[i - 1][1] > 0 {
                                knot_pos[i][1] -= 1;
                            } else {
                                knot_pos[i][1] += 1;
                            }
                        } else if (knot_pos[i][0] - knot_pos[i - 1][0]).abs() > 1 {
                            if knot_pos[i][0] - knot_pos[i - 1][0] > 0 {
                                knot_pos[i][0] -= 1;
                            } else {
                                knot_pos[i][0] += 1;
                            }
                            knot_pos[i][1] = knot_pos[i - 1][1];
                        } else if (knot_pos[i][1] - knot_pos[i - 1][1]).abs() > 1 {
                            if knot_pos[i][1] - knot_pos[i - 1][1] > 0 {
                                knot_pos[i][1] -= 1;
                            } else {
                                knot_pos[i][1] += 1;
                            }
                            knot_pos[i][0] = knot_pos[i - 1][0];
                        }

                        temp_pos = temp2_pos;
                    }
                }

                if !visited.contains(&knot_pos[knot_pos.len() - 1]) {
                    visited.push(knot_pos[knot_pos.len() - 1])
                }

                steps -= 1;
            }
        }
        Ok(visited.len().to_string())
    }
}
