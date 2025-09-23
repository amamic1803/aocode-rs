use crate::{Error, Solution};

day!(Day03, 2023, 3, "Gear Ratios");

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Schematic::new(input).sum_part_numbers().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Schematic::new(input).sum_of_gear_ratios().to_string())
    }
}

struct Schematic {
    scheme: Vec<Vec<char>>,
    generated_part_numbers: bool,
    part_numbers: Vec<(u64, usize, usize, usize)>, // (number, row, start, end)
}
impl Schematic {
    fn new(input: &str) -> Self {
        let scheme = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self {
            scheme,
            generated_part_numbers: false,
            part_numbers: Vec::new(),
        }
    }

    /// Get the character at the given coordinates.
    /// If the coordinates are out of bounds, return '.'.
    /// # Arguments
    /// * `i` - The i-th row.
    /// * `j` - The j-th col.
    /// # Returns
    /// The character at the given coordinates, or '.' if the coordinates are out of bounds.
    fn get(&self, i: isize, j: isize) -> char {
        if i < 0 || j < 0 {
            '.'
        } else {
            *self
                .scheme
                .get(i as usize)
                .unwrap_or(&vec![])
                .get(j as usize)
                .unwrap_or(&'.')
        }
    }

    fn generate_part_numbers(&mut self) {
        if self.generated_part_numbers {
            return;
        }

        for (i, row) in self.scheme.iter().enumerate() {
            let mut in_number = false;
            let mut start_index = 0;

            for (j, elem) in row.iter().enumerate() {
                if elem.is_ascii_digit() {
                    if !in_number {
                        in_number = true;
                        start_index = j;
                    }
                } else if in_number {
                    in_number = false;
                    let number = row[start_index..j]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();

                    if self.adjacent_symbols(i, start_index, j - 1) != 0 {
                        self.part_numbers.push((number, i, start_index, j - 1));
                    }
                }
            }

            if in_number {
                let number = row[start_index..]
                    .iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();

                if self.adjacent_symbols(i, start_index, row.len() - 1) != 0 {
                    self.part_numbers
                        .push((number, i, start_index, row.len() - 1));
                }
            }
        }

        self.generated_part_numbers = true;
    }

    fn sum_part_numbers(&mut self) -> u64 {
        self.generate_part_numbers();

        self.part_numbers
            .iter()
            .map(|(number, _, _, _)| number)
            .sum::<u64>()
    }

    fn adjacent_symbols(&self, row: usize, start: usize, end: usize) -> u64 {
        let mut count = 0;
        if self.get(row as isize, start as isize - 1) != '.' {
            count += 1;
        }
        if self.get(row as isize, end as isize + 1) != '.' {
            count += 1;
        }

        ((start as isize - 1)..=(end + 1) as isize).for_each(|x| {
            if self.get(row as isize - 1, x) != '.' {
                count += 1;
            }
        });
        ((start as isize - 1)..=(end + 1) as isize).for_each(|x| {
            if self.get(row as isize + 1, x) != '.' {
                count += 1;
            }
        });

        count
    }

    fn sum_of_gear_ratios(&mut self) -> u64 {
        self.generate_part_numbers();
        let mut sum = 0;

        for (i, row) in self.scheme.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if *elem == '*' {
                    let numbers_around = self.numbers_around(i, j);
                    if numbers_around.len() == 2 {
                        sum += numbers_around[0] * numbers_around[1];
                    }
                }
            }
        }

        sum
    }

    fn numbers_around(&self, i: usize, j: usize) -> Vec<u64> {
        let mut numbers = Vec::new();
        let i = i as isize;
        let j = j as isize;

        // left
        if j > 0 {
            for part_num in &self.part_numbers {
                if part_num.1 == i as usize && part_num.3 == (j - 1) as usize {
                    numbers.push(part_num.0);
                    break;
                }
            }
        }

        // right
        if (j as usize) < self.scheme[i as usize].len() - 1 {
            for part_num in &self.part_numbers {
                if part_num.1 == i as usize && part_num.2 == (j + 1) as usize {
                    numbers.push(part_num.0);
                    break;
                }
            }
        }

        // up
        if i > 0 {
            for part_num in &self.part_numbers {
                if part_num.1 == (i - 1) as usize
                    && ((part_num.2..=part_num.3).contains(&(j as usize))
                        || (part_num.2..=part_num.3).contains(&((j - 1) as usize))
                        || (part_num.2..=part_num.3).contains(&((j + 1) as usize)))
                {
                    numbers.push(part_num.0);
                }
            }
        }

        // down
        if (i as usize) < self.scheme.len() - 1 {
            for part_num in &self.part_numbers {
                if part_num.1 == (i + 1) as usize
                    && ((part_num.2..=part_num.3).contains(&(j as usize))
                        || (part_num.2..=part_num.3).contains(&((j - 1) as usize))
                        || (part_num.2..=part_num.3).contains(&((j + 1) as usize)))
                {
                    numbers.push(part_num.0);
                }
            }
        }

        numbers
    }
}
