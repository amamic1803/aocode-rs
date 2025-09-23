use crate::{Error, Solution};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

day!(Day03, 2019, 3, "Crossed Wires");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (line1, line2) = input.trim().split_once('\n').unwrap();
        let wire1 = Wire::new(line1);
        let wire2 = Wire::new(line2);
        let intersections = wire1.intersections(&wire2);

        Ok(intersections
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (line1, line2) = input.trim().split_once('\n').unwrap();
        let wire1 = Wire::new(line1);
        let wire2 = Wire::new(line2);
        let intersection_steps = wire1.intersections_steps(&wire2);

        Ok(intersection_steps
            .iter()
            .map(|(steps_1, steps_2)| steps_1 + steps_2)
            .min()
            .unwrap()
            .to_string())
    }
}

struct Wire {
    points: Vec<(i32, i32)>,
}
impl Wire {
    fn new(moves: &str) -> Self {
        let mut points = vec![(0, 0)];

        for next_move in moves.trim().split(',') {
            let (x, y) = points.last().unwrap();
            let (x, y) = match next_move.chars().next().unwrap() {
                'U' => (*x, y + next_move[1..].parse::<i32>().unwrap()),
                'D' => (*x, y - next_move[1..].parse::<i32>().unwrap()),
                'L' => (x - next_move[1..].parse::<i32>().unwrap(), *y),
                'R' => (x + next_move[1..].parse::<i32>().unwrap(), *y),
                _ => panic!("Invalid direction"),
            };
            points.push((x, y));
        }

        Self { points }
    }

    #[allow(clippy::collapsible_else_if)]
    fn intersections(&self, other: &Self) -> Vec<(i32, i32)> {
        let mut crossings = Vec::new();

        for i in 0..(self.points.len() - 1) {
            let self_pt1 = self.points[i];
            let self_pt2 = self.points[i + 1];
            for j in 0..(other.points.len() - 1) {
                let other_pt1 = other.points[j];
                let other_pt2 = other.points[j + 1];

                if self_pt1.0 == self_pt2.0 {
                    if other_pt1.0 == other_pt2.0 {
                        if self_pt1.0 == other_pt1.0 {
                            for k in self_pt1.1..=self_pt2.1 {
                                for l in other_pt1.1..=other_pt2.1 {
                                    if k == l {
                                        crossings.push((self_pt1.0, k));
                                    }
                                }
                            }
                        }
                    } else {
                        if other_pt1.1 >= min(self_pt1.1, self_pt2.1)
                            && other_pt1.1 <= max(self_pt1.1, self_pt2.1)
                            && self_pt1.0 >= min(other_pt1.0, other_pt2.0)
                            && self_pt1.0 <= max(other_pt1.0, other_pt2.0)
                        {
                            crossings.push((self_pt1.0, other_pt1.1));
                        }
                    }
                } else {
                    if other_pt1.0 == other_pt2.0 {
                        if other_pt1.0 >= min(self_pt1.0, self_pt2.0)
                            && other_pt1.0 <= max(self_pt1.0, self_pt2.0)
                            && self_pt1.1 >= min(other_pt1.1, other_pt2.1)
                            && self_pt1.1 <= max(other_pt1.1, other_pt2.1)
                        {
                            crossings.push((other_pt1.0, self_pt1.1));
                        }
                    } else {
                        if self_pt1.1 == other_pt1.1 {
                            for k in self_pt1.0..=self_pt2.0 {
                                for l in other_pt1.0..=other_pt2.0 {
                                    if k == l {
                                        crossings.push((k, self_pt1.1));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        crossings.retain(|(x, y)| (*x, *y) != (0, 0));

        crossings
    }

    fn intersections_steps(&self, other: &Self) -> Vec<(u32, u32)> {
        let mut intersections = HashSet::new();
        for inter in self.intersections(other) {
            intersections.insert(inter);
        }

        let wire1_steps = Self::measure_steps(&intersections, self);
        let wire2_steps = Self::measure_steps(&intersections, other);

        let mut steps = Vec::new();
        for inter in &intersections {
            steps.push((wire1_steps[inter], wire2_steps[inter]));
        }
        steps
    }

    #[allow(clippy::collapsible_else_if)]
    fn measure_steps(intersections: &HashSet<(i32, i32)>, wire: &Self) -> HashMap<(i32, i32), u32> {
        let mut steps = HashMap::new();
        let mut total_steps = 0;

        let mut check_insert = |point: (i32, i32), total_steps: u32| {
            if intersections.contains(&point) && !steps.contains_key(&point) {
                steps.insert(point, total_steps);
            }
        };

        for i in 0..(wire.points.len() - 1) {
            let point1 = wire.points[i];
            let point2 = wire.points[i + 1];

            if point1.0 == point2.0 {
                if point1.1 < point2.1 {
                    for point_y_coord in (point1.1 + 1)..=point2.1 {
                        total_steps += 1;
                        check_insert((point1.0, point_y_coord), total_steps);
                    }
                } else {
                    for point_y_coord in (point2.1..=(point1.1 - 1)).rev() {
                        total_steps += 1;
                        check_insert((point1.0, point_y_coord), total_steps);
                    }
                }
            } else {
                if point1.0 < point2.0 {
                    for point_x_coord in (point1.0 + 1)..=point2.0 {
                        total_steps += 1;
                        check_insert((point_x_coord, point1.1), total_steps);
                    }
                } else {
                    for point_x_coord in (point2.0..=(point1.0 - 1)).rev() {
                        total_steps += 1;
                        check_insert((point_x_coord, point1.1), total_steps);
                    }
                }
            }
        }
        steps
    }
}
