use crate::{Error, Solution};
use advent_of_code_ocr::parse_string_to_letters;

day!(Day10, 2022, 10, "Cathode-Ray Tube");

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut register_state = [1; 2];
        let mut result = 0;
        let wanted_cycles = [20, 60, 100, 140, 180, 220];
        let mut wanted_cycle_ind = 0;
        let mut cycle = 0;
        for command in input.trim().lines() {
            let mut command_split = command.split(' ');
            match command_split.next().unwrap() {
                "noop" => {
                    cycle += 1;
                }
                "addx" => {
                    cycle += 2;
                    register_state[0] = register_state[1];
                    register_state[1] += command_split.next().unwrap().parse::<i32>().unwrap();
                }
                _ => panic!(),
            }
            if cycle >= wanted_cycles[wanted_cycle_ind] - 1 {
                if cycle == wanted_cycles[wanted_cycle_ind] - 1 {
                    result += register_state[1] * wanted_cycles[wanted_cycle_ind];
                } else {
                    result += register_state[0] * wanted_cycles[wanted_cycle_ind];
                }
                wanted_cycle_ind += 1;
                if wanted_cycle_ind > wanted_cycles.len() - 1 {
                    break;
                }
            }
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut register_state: i32 = 1;
        let mut cycle: i32 = 0;
        let mut render: String = String::new();
        let mut commands = input.trim().lines();
        let mut prev_command: [i32; 2] = [0; 2];
        while cycle < 240 {
            if prev_command[0] == 0 {
                let mut command_split = commands.next().unwrap().split(' ');
                match command_split.next().unwrap() {
                    "noop" => {
                        prev_command[0] = 1;
                        register_state += prev_command[1];
                        prev_command[1] = 0;
                    }
                    "addx" => {
                        prev_command[0] = 2;
                        register_state += prev_command[1];
                        prev_command[1] = command_split.next().unwrap().parse::<i32>().unwrap();
                    }
                    _ => panic!(),
                }
            }

            if (register_state - (cycle % 40)).abs() < 2 {
                render.push('#');
            } else {
                render.push('.');
            }

            prev_command[0] -= 1;
            cycle += 1;
        }

        let mut result = String::new();
        let mut output: String = render;
        for _ in (40..241).step_by(40) {
            let splitt = output.split_at(40);
            result.push_str(splitt.0);
            result.push('\n');
            output = splitt.1.to_string();
        }

        Ok(parse_string_to_letters(result.trim()))
    }
}
