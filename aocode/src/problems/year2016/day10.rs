use crate::{Error, Solution};
use std::collections::VecDeque;

day!(Day10, 2016, 10, "Balance Bots");

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut processor = Processor::new(input);
        match processor.simulate(1) {
            Some(bot_id) => Ok(bot_id.to_string()),
            None => Err(Error::NoSolution),
        }
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut processor = Processor::new(input);
        match processor.simulate(2) {
            Some(product) => Ok(product.to_string()),
            None => Err(Error::NoSolution),
        }
    }
}

struct Processor {
    bots: Vec<Bot>,
    outputs: Vec<Out>,
}
impl Processor {
    fn new(input: &str) -> Self {
        let mut bots = Vec::new();
        let mut outputs = Vec::new();

        // (value, bot)
        let mut values: Vec<(u16, u16)> = Vec::new();

        for line in input.trim().lines() {
            let line_contents = line.split_whitespace().collect::<Vec<_>>();
            match line_contents[0] {
                "value" => values.push((
                    line_contents[1].parse().unwrap(),
                    line_contents[5].parse().unwrap(),
                )),
                "bot" => {
                    let bot_id = line_contents[1].parse().unwrap();
                    let low_id = line_contents[6].parse().unwrap();
                    let high_id = line_contents[11].parse().unwrap();
                    let low_pass_type = match line_contents[5] {
                        "bot" => PassType::Bot(low_id),
                        "output" => {
                            let output = Out::new(low_id, None);
                            if !outputs.contains(&output) {
                                outputs.push(output);
                            }
                            PassType::Out(low_id)
                        }
                        _ => panic!("Invalid input"),
                    };
                    let high_pass_type = match line_contents[10] {
                        "bot" => PassType::Bot(high_id),
                        "output" => {
                            let output = Out::new(high_id, None);
                            if !outputs.contains(&output) {
                                outputs.push(output);
                            }
                            PassType::Out(high_id)
                        }
                        _ => panic!("Invalid input"),
                    };

                    bots.push(Bot::new(bot_id, low_pass_type, high_pass_type));
                }
                _ => panic!("Invalid input"),
            }
        }

        for value in values {
            let bot = bots.iter_mut().find(|bot| bot.id == value.1).unwrap();
            bot.add_value(value.0);
        }

        bots.sort_by_key(|bot| bot.id);
        outputs.sort_by_key(|out| out.id);
        let bots_len = bots.len();
        let outputs_len = outputs.len();

        bots.dedup_by_key(|bot| bot.id);
        outputs.dedup_by_key(|out| out.id);

        assert_eq!(bots_len, bots.len());
        assert_eq!(outputs_len, outputs.len());

        assert_eq!(bots.len(), bots[bots.len() - 1].id as usize + 1);
        assert_eq!(outputs.len(), outputs[outputs.len() - 1].id as usize + 1);

        Self { bots, outputs }
    }

    fn simulate(&mut self, part: u8) -> Option<u64> {
        let mut current_bots: VecDeque<u16> = VecDeque::new();
        for bot in &self.bots {
            if bot.values() == 2 {
                current_bots.push_back(bot.id);
            }
        }

        while !current_bots.is_empty() {
            let step_count = current_bots.len();
            for _ in 0..step_count {
                let working_bot = self.bots[current_bots.pop_front().unwrap() as usize];
                let lower_value = working_bot
                    .values
                    .iter()
                    .filter(|value| value.is_some())
                    .min()
                    .unwrap()
                    .unwrap();
                let higher_value = working_bot
                    .values
                    .iter()
                    .filter(|value| value.is_some())
                    .max()
                    .unwrap()
                    .unwrap();

                if part == 1 && lower_value == 17 && higher_value == 61 {
                    return Some(working_bot.id as u64);
                }

                match working_bot.low {
                    PassType::Bot(id) => {
                        self.bots[id as usize].add_value(lower_value);
                        if self.bots[id as usize].values() == 2 {
                            current_bots.push_back(id);
                        }
                    }
                    PassType::Out(id) => self.outputs[id as usize].value = Some(lower_value),
                }

                match working_bot.high {
                    PassType::Bot(id) => {
                        self.bots[id as usize].add_value(higher_value);
                        if self.bots[id as usize].values() == 2 {
                            current_bots.push_back(id);
                        }
                    }
                    PassType::Out(id) => self.outputs[id as usize].value = Some(higher_value),
                }
            }
        }

        if part == 1 {
            None
        } else {
            let mut product = 1;

            for i in 0..3 {
                match self.outputs[i].value {
                    Some(value) => product *= value as u64,
                    None => return None,
                }
            }

            Some(product)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Bot {
    id: u16,
    low: PassType,
    high: PassType,
    values: [Option<u16>; 2],
}
impl Bot {
    fn new(id: u16, low: PassType, high: PassType) -> Self {
        Self {
            id,
            low,
            high,
            values: [None, None],
        }
    }

    fn add_value(&mut self, value: u16) {
        if self.values[0].is_none() {
            self.values[0] = Some(value);
        } else if self.values[1].is_none() {
            self.values[1] = Some(value);
        } else {
            panic!("Bot {} already has two values", self.id);
        }
    }

    fn values(&self) -> u8 {
        self.values.iter().filter(|value| value.is_some()).count() as u8
    }
}

#[derive(PartialEq, Eq)]
struct Out {
    id: u16,
    value: Option<u16>,
}
impl Out {
    fn new(id: u16, value: Option<u16>) -> Self {
        Self { id, value }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum PassType {
    Bot(u16),
    Out(u16),
}
