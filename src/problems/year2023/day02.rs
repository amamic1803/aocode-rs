use crate::{Error, Solution};

day!(Day02, 2023, 2, "Cube Conundrum");

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;
        let games = parse_input(input);

        for game in games {
            if game.is_possible() {
                sum += game.id;
            }
        }

        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;
        let games = parse_input(input);

        for game in games {
            let fewest_cubes = game.fewest_cubes();
            sum += fewest_cubes.iter().product::<u32>();
        }

        Ok(sum.to_string())
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.trim().lines() {
        let (game_id, draws) = line.split_once(':').unwrap();
        let game_id = game_id.trim_start_matches("Game ").parse::<u32>().unwrap();

        let mut game = Game::new(game_id);
        for draw in draws.split(';') {
            let mut values = [0; 3];

            for value in draw.split(',') {
                let val_num = value
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                if value.contains("green") {
                    values[1] = val_num;
                } else if value.contains("blue") {
                    values[2] = val_num;
                } else if value.contains("red") {
                    values[0] = val_num;
                } else {
                    panic!("Invalid color!");
                }
            }

            game.add_draw(values);
        }

        games.push(game);
    }

    games
}

const CUBE_AMOUNTS: [u32; 3] = [12, 13, 14]; // RGB

struct Game {
    id: u32,
    draws: Vec<[u32; 3]>, // RGB
}
impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }

    fn add_draw(&mut self, draw: [u32; 3]) {
        self.draws.push(draw);
    }

    fn is_possible(&self) -> bool {
        for draw in &self.draws {
            for (i, val) in draw.iter().enumerate() {
                if *val > CUBE_AMOUNTS[i] {
                    return false;
                }
            }
        }

        true
    }

    fn fewest_cubes(&self) -> [u32; 3] {
        let mut fewest_cubes = [0; 3];

        for draw in &self.draws {
            for (i, val) in draw.iter().enumerate() {
                if *val > fewest_cubes[i] {
                    fewest_cubes[i] = *val;
                }
            }
        }

        fewest_cubes
    }
}
