use crate::{Error, Solution};

day!(Day11, 2022, 11, "Monkey in the Middle");

impl Solution for Day11 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut monkeys = parse_monkeys(input);

        for _ in 0..20 {
            for monkey_ind in 0..monkeys.len() {
                monkeys[monkey_ind].inspected_items += monkeys[monkey_ind].items.len() as u128;

                for item_ind in 0..monkeys[monkey_ind].items.len() {
                    let new_data = monkeys[monkey_ind]
                        .throw(monkeys[monkey_ind].operation(monkeys[monkey_ind].items[item_ind]));
                    let new_item = new_data.0;
                    let new_location = new_data.1;
                    monkeys
                        .iter_mut()
                        .find(|m| m.id == new_location)
                        .unwrap()
                        .items
                        .push(new_item);
                }

                monkeys[monkey_ind].items.clear();
            }
        }

        let first_max = monkeys
            .iter()
            .max_by_key(|m| m.inspected_items)
            .unwrap()
            .inspected_items;
        let second_max = monkeys
            .iter()
            .filter(|m| m.inspected_items != first_max)
            .max_by_key(|m| m.inspected_items)
            .unwrap()
            .inspected_items;

        Ok((first_max * second_max).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut monkeys = parse_monkeys(input);

        let common_divisor: u128 = monkeys.iter().map(|m| m.throw[0]).product();

        for _ in 0..10_000 {
            for monkey_ind in 0..monkeys.len() {
                monkeys[monkey_ind].inspected_items += monkeys[monkey_ind].items.len() as u128;

                for item_ind in 0..monkeys[monkey_ind].items.len() {
                    let new_data = monkeys[monkey_ind]
                        .throw2(monkeys[monkey_ind].operation(monkeys[monkey_ind].items[item_ind]));
                    let new_item = new_data.0;
                    let new_location = new_data.1;
                    monkeys
                        .iter_mut()
                        .find(|m| m.id == new_location)
                        .unwrap()
                        .items
                        .push(new_item);
                }

                monkeys[monkey_ind].items.clear();
            }

            for monkey in &mut monkeys {
                monkey.items = monkey.items.iter().map(|i| i % common_divisor).collect();
            }
        }

        let first_max = monkeys
            .iter()
            .max_by_key(|m| m.inspected_items)
            .unwrap()
            .inspected_items;
        let second_max = monkeys
            .iter()
            .filter(|m| m.inspected_items != first_max)
            .max_by_key(|m| m.inspected_items)
            .unwrap()
            .inspected_items;

        Ok((first_max * second_max).to_string())
    }
}

fn parse_monkeys<'a>(input: &'a str) -> Vec<Monkey<'a>> {
    let mut line_contents: Vec<&str> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey = Monkey::new();
    for line in input.trim().lines() {
        line_contents.clear();
        line_contents.extend(line.trim().split(' '));
        match line_contents[0].trim_matches(':') {
            "" => {
                monkeys.push(current_monkey);
                current_monkey = Monkey::new();
            }
            "Monkey" => {
                current_monkey.id = line_contents[1].trim_matches(':').parse().unwrap();
            }
            "Starting" => {
                current_monkey.items = line_contents[2..]
                    .iter()
                    .map(|t| t.trim_matches(',').parse().unwrap())
                    .collect();
            }
            "Operation" => {
                current_monkey.operation_opts =
                    [line_contents[3], line_contents[4], line_contents[5]];
            }
            "Test" => current_monkey.throw[0] = line_contents[3].parse().unwrap(),
            "If" => {
                if line_contents[1].trim_matches(':') == "true" {
                    current_monkey.throw[1] = line_contents[5].trim_matches(',').parse().unwrap();
                } else {
                    current_monkey.throw[2] = line_contents[5].trim_matches(',').parse().unwrap();
                }
            }
            _ => panic!("unexpected line!"),
        }
    }
    monkeys.push(current_monkey);
    monkeys
}

struct Monkey<'a> {
    id: u128,
    items: Vec<u128>,
    operation_opts: [&'a str; 3],
    throw: [u128; 3], // div, true, false
    inspected_items: u128,
}

impl<'a> Monkey<'a> {
    fn new() -> Monkey<'a> {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation_opts: ["old", "+", "old"],
            throw: [0; 3],
            inspected_items: 0,
        }
    }

    fn operation(&self, x: u128) -> u128 {
        if self.operation_opts[1] == "+" {
            (if self.operation_opts[0] == "old" {
                x
            } else {
                self.operation_opts[0].parse().unwrap()
            }) + (if self.operation_opts[2] == "old" {
                x
            } else {
                self.operation_opts[2].parse().unwrap()
            })
        } else {
            (if self.operation_opts[0] == "old" {
                x
            } else {
                self.operation_opts[0].parse().unwrap()
            }) * (if self.operation_opts[2] == "old" {
                x
            } else {
                self.operation_opts[2].parse().unwrap()
            })
        }
    }

    fn throw(&self, x: u128) -> (u128, u128) {
        let temp = x / 3;
        (
            temp,
            if temp.is_multiple_of(self.throw[0]) {
                self.throw[1]
            } else {
                self.throw[2]
            },
        )
    }

    fn throw2(&self, x: u128) -> (u128, u128) {
        (
            x,
            if x.is_multiple_of(self.throw[0]) {
                self.throw[1]
            } else {
                self.throw[2]
            },
        )
    }
}
