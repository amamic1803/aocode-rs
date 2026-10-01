use crate::{Error, Solution};

day!(Day15, 2015, 15, "Science for Hungry People");

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let ingredients = parse_input(input);

        let mut max = isize::MIN;
        for i in 0..101 {
            for j in 0..(101 - i) {
                for k in 0..(101 - i - j) {
                    let l = 100 - i - j - k;
                    let capacity = i * ingredients[0][0]
                        + j * ingredients[1][0]
                        + k * ingredients[2][0]
                        + l * ingredients[3][0];
                    let durability = i * ingredients[0][1]
                        + j * ingredients[1][1]
                        + k * ingredients[2][1]
                        + l * ingredients[3][1];
                    let flavor = i * ingredients[0][2]
                        + j * ingredients[1][2]
                        + k * ingredients[2][2]
                        + l * ingredients[3][2];
                    let texture = i * ingredients[0][3]
                        + j * ingredients[1][3]
                        + k * ingredients[2][3]
                        + l * ingredients[3][3];

                    let value = if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                        0
                    } else {
                        capacity * durability * flavor * texture
                    };

                    if value > max {
                        max = value;
                    }
                }
            }
        }

        Ok(max.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let ingredients = parse_input(input);

        let mut max = isize::MIN;
        for i in 0..101 {
            for j in 0..(101 - i) {
                for k in 0..(101 - i - j) {
                    let l = 100 - i - j - k;

                    let calories = i * ingredients[0][4]
                        + j * ingredients[1][4]
                        + k * ingredients[2][4]
                        + l * ingredients[3][4];
                    if calories != 500 {
                        continue;
                    }

                    let capacity = i * ingredients[0][0]
                        + j * ingredients[1][0]
                        + k * ingredients[2][0]
                        + l * ingredients[3][0];
                    let durability = i * ingredients[0][1]
                        + j * ingredients[1][1]
                        + k * ingredients[2][1]
                        + l * ingredients[3][1];
                    let flavor = i * ingredients[0][2]
                        + j * ingredients[1][2]
                        + k * ingredients[2][2]
                        + l * ingredients[3][2];
                    let texture = i * ingredients[0][3]
                        + j * ingredients[1][3]
                        + k * ingredients[2][3]
                        + l * ingredients[3][3];

                    let value = if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                        0
                    } else {
                        capacity * durability * flavor * texture
                    };

                    if value > max {
                        max = value;
                    }
                }
            }
        }

        Ok(max.to_string())
    }
}

fn parse_input(input: &str) -> Vec<[isize; 5]> {
    let mut ingredients = Vec::new();

    for line in input.trim().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut line_array = [0; 5];
        line_array[0] = words[2].trim_end_matches(',').parse::<isize>().unwrap();
        line_array[1] = words[4].trim_end_matches(',').parse::<isize>().unwrap();
        line_array[2] = words[6].trim_end_matches(',').parse::<isize>().unwrap();
        line_array[3] = words[8].trim_end_matches(',').parse::<isize>().unwrap();
        line_array[4] = words[10].trim_end_matches(',').parse::<isize>().unwrap();

        ingredients.push(line_array);
    }

    assert_eq!(ingredients.len(), 4);

    ingredients
}
