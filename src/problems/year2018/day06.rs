use crate::math::manhattan_distance;
use crate::{Error, Solution};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

day!(Day06, 2018, 6, "Chronal Coordinates");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let coords = parse_input(input);
        let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();

        let mut grid = vec![vec![None; max_y + 1]; max_x + 1];
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let mut min_index = 0;
                let mut min_count = 1;
                let mut min_dist = manhattan_distance(
                    (i as i64, j as i64),
                    (coords[0].0 as i64, coords[0].1 as i64),
                );

                for (k, coord) in coords.iter().enumerate().skip(1) {
                    let dist =
                        manhattan_distance((i as i64, j as i64), (coord.0 as i64, coord.1 as i64));
                    match dist.cmp(&min_dist) {
                        Ordering::Less => {
                            min_index = k;
                            min_count = 1;
                            min_dist = dist;
                        }
                        Ordering::Equal => {
                            min_count += 1;
                        }
                        Ordering::Greater => {}
                    }
                }

                if min_count == 1 {
                    *cell = Some(min_index);
                }
            }
        }

        let mut infinite_areas = HashSet::new();
        for cell in grid.first().unwrap().iter().flatten() {
            infinite_areas.insert(*cell);
        }
        for cell in grid.last().unwrap().iter().flatten() {
            infinite_areas.insert(*cell);
        }
        for row in grid.iter() {
            if let Some(cell) = row.first().unwrap() {
                infinite_areas.insert(*cell);
            }
            if let Some(cell) = row.last().unwrap() {
                infinite_areas.insert(*cell);
            }
        }

        let mut areas = HashMap::new();
        for row in grid.iter() {
            for cell in row.iter().flatten() {
                if !infinite_areas.contains(cell) {
                    *areas.entry(cell).or_insert(0) += 1;
                }
            }
        }

        Ok(areas.into_values().max().unwrap().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let coords = parse_input(input);
        let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();

        let mut area_size = 0;
        for i in 0..=max_x {
            for j in 0..=max_y {
                let mut total_distances = 0;
                for coord in coords.iter() {
                    let dist =
                        manhattan_distance((i as i64, j as i64), (coord.0 as i64, coord.1 as i64));
                    total_distances += dist;
                    if total_distances >= PART2_THRESHOLD {
                        break;
                    }
                }
                if total_distances < PART2_THRESHOLD {
                    area_size += 1;
                }
            }
        }

        Ok(area_size.to_string())
    }
}

const PART2_THRESHOLD: u64 = 10000;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect()
}
