use crate::{Error, Solution};
use advent_of_code_ocr::parse_string_to_letters;
use itertools::Itertools;

day!(Day08, 2016, 8, "Two-Factor Authentication");

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let screen = simulate_screen(input);
        Ok(screen.pixels_on().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let screen = simulate_screen(input);
        let mut output = String::new();
        for row in screen.pixels {
            for pixel in row {
                if pixel {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        Ok(parse_string_to_letters(&output))
    }
}

fn simulate_screen(input: &str) -> Screen {
    let mut screen = Screen::new();

    for line in input.trim().lines() {
        let line_contents = line.split_whitespace().collect::<Vec<&str>>();
        match line_contents[0] {
            "rect" => {
                let (x, y) = line_contents[1]
                    .split('x')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                screen.rectangle(x, y);
            }
            "rotate" => {
                let n_th = line_contents[2].split('=').collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let by = line_contents[4].parse::<usize>().unwrap();
                match line_contents[1] {
                    "row" => screen.rotate_row(n_th, by),
                    "column" => screen.rotate_col(n_th, by),
                    _ => panic!("Invalid instruction"),
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }

    screen
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: vec![vec![false; 50]; 6],
        }
    }

    fn transpose(&mut self) {
        let mut new_pixels: Vec<Vec<bool>> = Vec::new();
        for col_num in 0..self.pixels[0].len() {
            let mut new_row: Vec<bool> = Vec::new();
            for row_num in 0..self.pixels.len() {
                new_row.push(self.pixels[row_num][col_num]);
            }
            new_pixels.push(new_row);
        }
        self.pixels = new_pixels;
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        self.pixels[row].rotate_right(amount);
    }

    fn rotate_col(&mut self, col: usize, amount: usize) {
        self.transpose();
        self.rotate_row(col, amount);
        self.transpose();
    }

    fn rectangle(&mut self, x: usize, y: usize) {
        for row in 0..y {
            for col in 0..x {
                self.pixels[row][col] = true;
            }
        }
    }

    fn pixels_on(&self) -> usize {
        let mut count = 0;
        for row in &self.pixels {
            for pixel in row {
                if *pixel {
                    count += 1;
                }
            }
        }
        count
    }
}
